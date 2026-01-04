//! OAuth 认证模块
//!
//! 实现 Claude OAuth 2.0 + PKCE 认证流程

use crate::credentials::{OAuthParams, OAuthTokens};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::{Duration, Utc};
use rand::Rng;
use reqwest::Client;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tracing::{debug, info};

/// OAuth 配置常量
pub const CLAUDE_AUTH_URL: &str = "https://claude.ai/oauth/authorize";
pub const CLAUDE_TOKEN_URL: &str = "https://console.anthropic.com/v1/oauth/token";
pub const CLAUDE_CLIENT_ID: &str = "9d1c250a-e61b-44d9-88ed-5944d1962f5e";
pub const CLAUDE_REDIRECT_URI: &str = "https://console.anthropic.com/oauth/code/callback";
pub const CLAUDE_SCOPES: &str = "org:create_api_key user:profile user:inference";
pub const CLAUDE_SCOPES_SETUP: &str = "user:inference";

/// Token 响应
#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Option<i64>,
    #[allow(dead_code)]
    token_type: Option<String>,
    account: Option<AccountInfo>,
}

/// 账户信息
#[derive(Debug, Deserialize)]
struct AccountInfo {
    email_address: Option<String>,
    #[allow(dead_code)]
    uuid: Option<String>,
}

/// 组织信息
#[derive(Debug, Deserialize)]
struct Organization {
    #[allow(dead_code)]
    uuid: String,
    name: String,
    capabilities: Vec<String>,
}

/// 生成 OAuth 参数（PKCE）
pub fn generate_oauth_params(is_setup_token: bool) -> OAuthParams {
    // 1. 生成随机 state
    let state_bytes: [u8; 32] = rand::thread_rng().gen();
    let state = URL_SAFE_NO_PAD.encode(state_bytes);

    // 2. 生成 code_verifier
    let verifier_bytes: [u8; 32] = rand::thread_rng().gen();
    let code_verifier = URL_SAFE_NO_PAD.encode(verifier_bytes);

    // 3. 计算 code_challenge = SHA256(code_verifier)
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let code_challenge = URL_SAFE_NO_PAD.encode(hasher.finalize());

    // 4. 选择 scopes
    let scopes = if is_setup_token {
        CLAUDE_SCOPES_SETUP
    } else {
        CLAUDE_SCOPES
    };

    // 5. 构建授权 URL
    let auth_url = format!(
        "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
        CLAUDE_AUTH_URL,
        CLAUDE_CLIENT_ID,
        urlencoding::encode(CLAUDE_REDIRECT_URI),
        urlencoding::encode(scopes),
        state,
        code_challenge
    );

    OAuthParams {
        auth_url,
        code_verifier,
        state,
        code_challenge,
    }
}

/// 交换授权码获取 Token
pub async fn exchange_authorization_code(
    authorization_code: &str,
    code_verifier: &str,
    state: &str,
) -> Result<OAuthTokens> {
    let client = Client::builder()
        .connect_timeout(std::time::Duration::from_secs(30))
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    debug!("交换授权码: code={}", &authorization_code[..20.min(authorization_code.len())]);

    let response = client
        .post(CLAUDE_TOKEN_URL)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "client_id": CLAUDE_CLIENT_ID,
            "grant_type": "authorization_code",
            "code": authorization_code,
            "redirect_uri": CLAUDE_REDIRECT_URI,
            "code_verifier": code_verifier,
            "state": state
        }))
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("Token 交换失败: {} - {}", status, body);
    }

    let token_response: TokenResponse = response.json().await?;

    let expires_at = token_response
        .expires_in
        .map(|secs| Utc::now() + Duration::seconds(secs));

    info!("OAuth Token 交换成功");

    Ok(OAuthTokens {
        access_token: token_response.access_token,
        refresh_token: token_response.refresh_token,
        expires_at,
        email: token_response.account.and_then(|a| a.email_address),
    })
}

/// 使用 sessionKey 自动完成 OAuth 流程
pub async fn oauth_with_cookie(session_key: &str, is_setup_token: bool) -> Result<OAuthTokens> {
    let client = Client::builder()
        .connect_timeout(std::time::Duration::from_secs(30))
        .timeout(std::time::Duration::from_secs(60))
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    info!("使用 Cookie 进行 OAuth 授权");

    // 1. 获取组织信息
    let orgs_response = client
        .get("https://claude.ai/api/organizations")
        .header("Cookie", format!("sessionKey={}", session_key))
        .header(
            "User-Agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36",
        )
        .header("Origin", "https://claude.ai")
        .header("Referer", "https://claude.ai/new")
        .send()
        .await?;

    let status = orgs_response.status();
    if !status.is_success() {
        let body = orgs_response.text().await.unwrap_or_default();
        anyhow::bail!("获取组织信息失败: {} - {}", status, body);
    }

    let organizations: Vec<Organization> = orgs_response.json().await?;

    // 2. 选择具有 chat 能力的组织
    let _org = organizations
        .iter()
        .find(|o| o.capabilities.contains(&"chat".to_string()))
        .ok_or_else(|| anyhow::anyhow!("没有找到具有 chat 能力的组织"))?;

    debug!("找到有效组织: {}", _org.name);

    // 3. 生成 OAuth 参数
    let params = generate_oauth_params(is_setup_token);

    // 4. 使用 Cookie 请求授权码
    let auth_response = client
        .get(&params.auth_url)
        .header("Cookie", format!("sessionKey={}", session_key))
        .header(
            "User-Agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36",
        )
        .send()
        .await?;

    // 5. 解析回调中的授权码
    let callback_url = auth_response
        .headers()
        .get("location")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| anyhow::anyhow!("未收到重定向响应"))?;

    let code = extract_code_from_url(callback_url)?;

    debug!("获取到授权码");

    // 6. 交换 Token
    exchange_authorization_code(&code, &params.code_verifier, &params.state).await
}

/// 从 URL 中提取授权码
fn extract_code_from_url(url: &str) -> Result<String> {
    let url = reqwest::Url::parse(url)?;
    let code = url
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, value)| value.to_string())
        .ok_or_else(|| anyhow::anyhow!("URL 中没有找到授权码"))?;
    Ok(code)
}

/// 刷新 OAuth Token
pub async fn refresh_oauth_token(refresh_token: &str) -> Result<OAuthTokens> {
    let client = Client::builder()
        .connect_timeout(std::time::Duration::from_secs(30))
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    debug!("刷新 OAuth Token");

    let response = client
        .post(CLAUDE_TOKEN_URL)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "client_id": CLAUDE_CLIENT_ID,
            "grant_type": "refresh_token",
            "refresh_token": refresh_token
        }))
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("Token 刷新失败: {} - {}", status, body);
    }

    let token_response: TokenResponse = response.json().await?;

    let expires_at = token_response
        .expires_in
        .map(|secs| Utc::now() + Duration::seconds(secs));

    info!("OAuth Token 刷新成功");

    Ok(OAuthTokens {
        access_token: token_response.access_token,
        refresh_token: token_response.refresh_token,
        expires_at,
        email: token_response.account.and_then(|a| a.email_address),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_oauth_params() {
        let params = generate_oauth_params(false);
        assert!(params.auth_url.contains("claude.ai/oauth/authorize"));
        assert!(params.auth_url.contains("code_challenge_method=S256"));
        assert!(!params.code_verifier.is_empty());
        assert!(!params.state.is_empty());
        assert!(!params.code_challenge.is_empty());
    }

    #[test]
    fn test_generate_setup_token_params() {
        let params = generate_oauth_params(true);
        assert!(params.auth_url.contains("user%3Ainference"));
        assert!(!params.auth_url.contains("org%3Acreate_api_key"));
    }
}
