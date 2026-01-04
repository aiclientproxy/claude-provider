//! Token 刷新逻辑
//!
//! 支持 OAuth、Claude Code、Console 等认证方式的 Token 刷新

#![allow(dead_code)]

use crate::auth::oauth::refresh_oauth_token;
use crate::credentials::{AuthType, ClaudeCredentials};
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

/// Token 刷新结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRefreshResult {
    /// 新的 access_token
    pub access_token: String,
    /// 新的 refresh_token（如果更新了）
    #[serde(default)]
    pub refresh_token: Option<String>,
    /// 过期时间
    #[serde(default)]
    pub expires_at: Option<DateTime<Utc>>,
    /// 邮箱
    #[serde(default)]
    pub email: Option<String>,
}

/// 刷新 Token
pub async fn refresh_token(credential: &mut ClaudeCredentials) -> Result<TokenRefreshResult> {
    match credential.auth_type {
        AuthType::OAuth | AuthType::ClaudeCode | AuthType::Console => {
            refresh_oauth_based_token(credential).await
        }
        AuthType::SetupToken => {
            // Setup Token 没有 refresh_token，无法刷新
            anyhow::bail!("Setup Token 不支持刷新，请重新授权")
        }
        AuthType::Bedrock => {
            // Bedrock 使用 AWS 凭证，不需要刷新
            anyhow::bail!("Bedrock 凭证不需要刷新")
        }
        AuthType::Ccr => {
            // CCR 使用 API Key，不需要刷新
            anyhow::bail!("CCR 凭证不需要刷新")
        }
    }
}

/// 刷新 OAuth 类型的 Token
async fn refresh_oauth_based_token(credential: &mut ClaudeCredentials) -> Result<TokenRefreshResult> {
    // 验证 refresh_token 存在
    let refresh_token = credential
        .refresh_token
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("缺少 refresh_token"))?;

    // 验证 refresh_token 完整性
    if refresh_token.len() < 50 {
        anyhow::bail!(
            "refresh_token 已被截断（长度: {} 字符）。正常的 refresh_token 长度应该更长",
            refresh_token.len()
        );
    }

    info!(
        "开始 Token 刷新: auth_type={}",
        credential.auth_type
    );

    // 调用 OAuth 刷新
    let tokens = refresh_oauth_token(refresh_token).await?;

    // 更新凭证
    credential.access_token = Some(tokens.access_token.clone());
    if let Some(ref rt) = tokens.refresh_token {
        credential.refresh_token = Some(rt.clone());
    }
    credential.expire = tokens.expires_at.map(|dt| dt.to_rfc3339());
    credential.last_refresh = Some(Utc::now().to_rfc3339());
    credential.is_healthy = true;
    credential.last_error = None;

    if let Some(ref email) = tokens.email {
        credential.email = Some(email.clone());
    }

    info!("Token 刷新成功");

    Ok(TokenRefreshResult {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_at: tokens.expires_at,
        email: tokens.email,
    })
}

/// 检查 Token 是否已过期
pub fn is_token_expired(expire: Option<&str>) -> bool {
    if let Some(expire_str) = expire {
        if let Ok(expires) = DateTime::parse_from_rfc3339(expire_str) {
            let now = Utc::now();
            // 提前5分钟判断为过期
            return expires <= now + Duration::minutes(5);
        }
    }
    // 如果没有过期时间信息，保守地认为可能需要刷新
    true
}

/// 检查 Token 是否即将过期（10 分钟内）
pub fn is_token_expiring_soon(expire: Option<&str>) -> bool {
    if let Some(expire_str) = expire {
        if let Ok(expiry) = DateTime::parse_from_rfc3339(expire_str) {
            let now = Utc::now();
            let threshold = now + Duration::minutes(10);
            return expiry < threshold;
        }
    }
    false
}

/// 带重试的 Token 刷新
pub async fn refresh_token_with_retry(
    credential: &mut ClaudeCredentials,
    max_retries: u32,
) -> Result<TokenRefreshResult> {
    let mut last_error = None;

    for attempt in 0..max_retries {
        match refresh_token(credential).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                warn!("Token 刷新失败 (尝试 {}/{}): {}", attempt + 1, max_retries, e);
                last_error = Some(e);
                // 指数退避
                let delay = std::time::Duration::from_millis(1000 * 2_u64.pow(attempt));
                tokio::time::sleep(delay).await;
            }
        }
    }

    Err(last_error.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_token_expired() {
        // 已过期
        let expired = (Utc::now() - Duration::hours(1)).to_rfc3339();
        assert!(is_token_expired(Some(&expired)));

        // 即将过期（5分钟内）
        let expiring = (Utc::now() + Duration::minutes(3)).to_rfc3339();
        assert!(is_token_expired(Some(&expiring)));

        // 未过期
        let valid = (Utc::now() + Duration::hours(1)).to_rfc3339();
        assert!(!is_token_expired(Some(&valid)));

        // 无过期时间
        assert!(is_token_expired(None));
    }

    #[test]
    fn test_is_token_expiring_soon() {
        // 10分钟内过期
        let expiring = (Utc::now() + Duration::minutes(5)).to_rfc3339();
        assert!(is_token_expiring_soon(Some(&expiring)));

        // 超过10分钟
        let valid = (Utc::now() + Duration::hours(1)).to_rfc3339();
        assert!(!is_token_expiring_soon(Some(&valid)));
    }
}
