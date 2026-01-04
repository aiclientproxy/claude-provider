//! CCR (Custom Claude Relay) 认证模块
//!
//! 实现第三方 Claude 中转服务的认证和调用

#![allow(dead_code)]

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::debug;

/// CCR 凭证
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CCRCredentials {
    pub api_key: String,
    pub base_url: String,
    pub name: Option<String>,
}

/// 验证 CCR 凭证
pub async fn validate_ccr_credentials(credentials: &CCRCredentials) -> Result<bool> {
    let client = Client::builder()
        .connect_timeout(std::time::Duration::from_secs(10))
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    // 尝试调用 /v1/models 端点验证凭证
    let url = format!("{}/v1/models", credentials.base_url.trim_end_matches('/'));

    debug!("验证 CCR 凭证: {}", url);

    let response = client
        .get(&url)
        .header("x-api-key", &credentials.api_key)
        .header("anthropic-version", "2023-06-01")
        .send()
        .await?;

    // 如果返回 401/403 则凭证无效，其他状态码可能是端点不存在但凭证有效
    let status = response.status();
    if status.as_u16() == 401 || status.as_u16() == 403 {
        return Ok(false);
    }

    Ok(true)
}

/// 构建 CCR API URL
pub fn build_ccr_url(base_url: &str, endpoint: &str) -> String {
    format!(
        "{}/{}",
        base_url.trim_end_matches('/'),
        endpoint.trim_start_matches('/')
    )
}

/// 构建 CCR 请求头
pub fn build_ccr_headers(api_key: &str) -> Vec<(&'static str, String)> {
    vec![
        ("x-api-key", api_key.to_string()),
        ("anthropic-version", "2023-06-01".to_string()),
        ("Content-Type", "application/json".to_string()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_ccr_url() {
        assert_eq!(
            build_ccr_url("https://api.example.com", "v1/messages"),
            "https://api.example.com/v1/messages"
        );
        assert_eq!(
            build_ccr_url("https://api.example.com/", "/v1/messages"),
            "https://api.example.com/v1/messages"
        );
    }

    #[test]
    fn test_build_ccr_headers() {
        let headers = build_ccr_headers("test-api-key");
        assert_eq!(headers.len(), 3);
        assert_eq!(headers[0], ("x-api-key", "test-api-key".to_string()));
    }
}
