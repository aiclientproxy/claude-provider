//! Claude Provider 核心实现
//!
//! 实现凭证管理、模型支持检查等核心功能。

use crate::credentials::{AcquiredCredential, AuthType, ClaudeCredentials, ValidationResult};
use crate::token_refresh::TokenRefreshResult;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// 模型信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub display_name: String,
    pub family: Option<String>,
    pub context_length: Option<u32>,
    pub supports_vision: bool,
    pub supports_tools: bool,
}

/// Provider 错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderError {
    pub error_type: String,
    pub message: String,
    pub status_code: Option<u16>,
    pub retryable: bool,
    pub cooldown_seconds: Option<u64>,
}

lazy_static::lazy_static! {
    static ref CREDENTIALS: Arc<RwLock<HashMap<String, ClaudeCredentials>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

/// 列出支持的模型
pub fn list_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "claude-opus-4-20250514".to_string(),
            display_name: "Claude Opus 4".to_string(),
            family: Some("opus".to_string()),
            context_length: Some(200000),
            supports_vision: true,
            supports_tools: true,
        },
        ModelInfo {
            id: "claude-opus-4-5-20251101".to_string(),
            display_name: "Claude Opus 4.5".to_string(),
            family: Some("opus".to_string()),
            context_length: Some(200000),
            supports_vision: true,
            supports_tools: true,
        },
        ModelInfo {
            id: "claude-sonnet-4-20250514".to_string(),
            display_name: "Claude Sonnet 4".to_string(),
            family: Some("sonnet".to_string()),
            context_length: Some(200000),
            supports_vision: true,
            supports_tools: true,
        },
        ModelInfo {
            id: "claude-sonnet-4-5-20250929".to_string(),
            display_name: "Claude Sonnet 4.5".to_string(),
            family: Some("sonnet".to_string()),
            context_length: Some(200000),
            supports_vision: true,
            supports_tools: true,
        },
        ModelInfo {
            id: "claude-haiku-3-5-20241022".to_string(),
            display_name: "Claude Haiku 3.5".to_string(),
            family: Some("haiku".to_string()),
            context_length: Some(200000),
            supports_vision: true,
            supports_tools: true,
        },
        ModelInfo {
            id: "claude-3-5-sonnet-20241022".to_string(),
            display_name: "Claude 3.5 Sonnet".to_string(),
            family: Some("sonnet".to_string()),
            context_length: Some(200000),
            supports_vision: true,
            supports_tools: true,
        },
    ]
}

/// 检查是否支持某个模型
pub fn supports_model(model: &str) -> bool {
    model.starts_with("claude-")
}

/// 获取凭证
pub async fn acquire_credential(model: &str) -> Result<AcquiredCredential> {
    if !supports_model(model) {
        anyhow::bail!("不支持的模型: {}", model);
    }

    let creds = CREDENTIALS.read().await;

    // 查找健康的凭证
    let healthy_creds: Vec<_> = creds.iter().filter(|(_, c)| c.is_healthy).collect();

    if healthy_creds.is_empty() {
        anyhow::bail!("没有可用的健康凭证");
    }

    // 选择第一个健康凭证
    let (id, credential) = healthy_creds.first().unwrap();

    // 根据认证类型构建请求头和 base_url
    let (base_url, headers) = match credential.auth_type {
        AuthType::OAuth | AuthType::ClaudeCode | AuthType::Console | AuthType::SetupToken => {
            let token = credential
                .access_token
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("凭证没有有效的 access_token"))?;

            let mut headers = HashMap::new();
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
            headers.insert("Content-Type".to_string(), "application/json".to_string());
            headers.insert("anthropic-version".to_string(), "2023-06-01".to_string());

            (Some("https://api.anthropic.com".to_string()), headers)
        }
        AuthType::Bedrock => {
            // Bedrock 需要 AWS 签名，这里只返回基本信息
            let region = credential.region.as_deref().unwrap_or("us-east-1");
            let base_url = format!("https://bedrock-runtime.{}.amazonaws.com", region);

            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "application/json".to_string());

            (Some(base_url), headers)
        }
        AuthType::Ccr => {
            let api_key = credential
                .api_key
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("CCR 凭证没有 api_key"))?;
            let base_url = credential
                .base_url
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("CCR 凭证没有 base_url"))?;

            let mut headers = HashMap::new();
            headers.insert("x-api-key".to_string(), api_key.clone());
            headers.insert("Content-Type".to_string(), "application/json".to_string());
            headers.insert("anthropic-version".to_string(), "2023-06-01".to_string());

            (Some(base_url.clone()), headers)
        }
    };

    Ok(AcquiredCredential {
        id: (*id).clone(),
        name: credential.name.clone(),
        auth_type: credential.auth_type.to_string(),
        base_url,
        headers,
        metadata: HashMap::new(),
    })
}

/// 释放凭证
pub async fn release_credential(credential_id: &str, result: serde_json::Value) -> Result<()> {
    let mut creds = CREDENTIALS.write().await;

    if let Some(credential) = creds.get_mut(credential_id) {
        credential.usage_count += 1;

        if let Some(error) = result.get("error") {
            credential.error_count += 1;
            credential.last_error = error
                .get("message")
                .and_then(|m| m.as_str())
                .map(String::from);

            if error
                .get("mark_unhealthy")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            {
                credential.is_healthy = false;
                warn!("凭证标记为不健康: {}", credential_id);
            }
        } else {
            credential.is_healthy = true;
            credential.last_error = None;
            debug!("凭证使用成功: {}", credential_id);
        }
    }

    Ok(())
}

/// 验证凭证
pub async fn validate_credential(credential_id: &str) -> Result<ValidationResult> {
    let creds = CREDENTIALS.read().await;

    if let Some(credential) = creds.get(credential_id) {
        let is_valid = match credential.auth_type {
            AuthType::OAuth | AuthType::ClaudeCode | AuthType::Console | AuthType::SetupToken => {
                credential.access_token.is_some()
            }
            AuthType::Bedrock => {
                credential.access_key_id.is_some() && credential.secret_access_key.is_some()
            }
            AuthType::Ccr => credential.api_key.is_some() && credential.base_url.is_some(),
        };

        Ok(ValidationResult {
            valid: is_valid && credential.is_healthy,
            message: if is_valid {
                Some("凭证有效".to_string())
            } else {
                Some("凭证配置不完整".to_string())
            },
            details: HashMap::new(),
        })
    } else {
        Ok(ValidationResult {
            valid: false,
            message: Some("凭证不存在".to_string()),
            details: HashMap::new(),
        })
    }
}

/// 刷新 Token
pub async fn refresh_token(credential_id: &str) -> Result<TokenRefreshResult> {
    let mut creds = CREDENTIALS.write().await;

    if let Some(credential) = creds.get_mut(credential_id) {
        // 调用 token_refresh 模块
        let result = crate::token_refresh::refresh_token(credential).await?;

        info!("Token 刷新成功: {}", credential_id);
        Ok(result)
    } else {
        anyhow::bail!("凭证不存在: {}", credential_id)
    }
}

/// 创建凭证
pub async fn create_credential(auth_type: &str, config: serde_json::Value) -> Result<String> {
    let auth_type_enum = match auth_type {
        "oauth" => AuthType::OAuth,
        "claude_code" => AuthType::ClaudeCode,
        "console" => AuthType::Console,
        "setup_token" => AuthType::SetupToken,
        "bedrock" => AuthType::Bedrock,
        "ccr" => AuthType::Ccr,
        _ => anyhow::bail!("不支持的认证类型: {}", auth_type),
    };

    let mut claude_config: ClaudeCredentials = serde_json::from_value(config)?;
    claude_config.auth_type = auth_type_enum;

    // 验证必要字段
    match auth_type_enum {
        AuthType::OAuth | AuthType::ClaudeCode | AuthType::Console => {
            if claude_config.refresh_token.is_none() && claude_config.access_token.is_none() {
                anyhow::bail!("OAuth 类型凭证需要 access_token 或 refresh_token");
            }
        }
        AuthType::SetupToken => {
            if claude_config.access_token.is_none() {
                anyhow::bail!("Setup Token 需要 access_token");
            }
        }
        AuthType::Bedrock => {
            if claude_config.access_key_id.is_none() || claude_config.secret_access_key.is_none() {
                anyhow::bail!("Bedrock 凭证需要 access_key_id 和 secret_access_key");
            }
        }
        AuthType::Ccr => {
            if claude_config.api_key.is_none() || claude_config.base_url.is_none() {
                anyhow::bail!("CCR 凭证需要 api_key 和 base_url");
            }
        }
    }

    // 生成凭证 ID
    let credential_id = uuid::Uuid::new_v4().to_string();

    // 存储凭证
    let mut creds = CREDENTIALS.write().await;
    creds.insert(credential_id.clone(), claude_config);

    info!("创建凭证成功: {} (类型: {})", credential_id, auth_type);
    Ok(credential_id)
}

/// 转换请求
pub async fn transform_request(request: serde_json::Value) -> Result<serde_json::Value> {
    // Claude Provider 直接使用 Anthropic 格式，无需转换
    Ok(request)
}

/// 转换响应
pub async fn transform_response(response: serde_json::Value) -> Result<serde_json::Value> {
    // 响应转换
    Ok(response)
}

/// 应用风控
pub async fn apply_risk_control(
    _request: &mut serde_json::Value,
    _credential_id: &str,
) -> Result<()> {
    // Claude Provider 暂不需要特殊风控
    Ok(())
}

/// 解析错误
pub fn parse_error(status: u16, body: &str) -> Option<ProviderError> {
    match status {
        401 => Some(ProviderError {
            error_type: "authentication".to_string(),
            message: "Token 已过期或无效".to_string(),
            status_code: Some(status),
            retryable: true,
            cooldown_seconds: Some(0),
        }),
        403 => Some(ProviderError {
            error_type: "authorization".to_string(),
            message: "权限不足".to_string(),
            status_code: Some(status),
            retryable: false,
            cooldown_seconds: None,
        }),
        429 => Some(ProviderError {
            error_type: "rate_limit".to_string(),
            message: "请求过于频繁".to_string(),
            status_code: Some(status),
            retryable: true,
            cooldown_seconds: Some(60),
        }),
        500..=599 => Some(ProviderError {
            error_type: "server_error".to_string(),
            message: format!("服务器错误: {}", body),
            status_code: Some(status),
            retryable: true,
            cooldown_seconds: Some(10),
        }),
        _ => None,
    }
}
