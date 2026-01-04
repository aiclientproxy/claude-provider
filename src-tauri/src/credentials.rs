//! 凭证数据结构

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 认证类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthType {
    /// 标准 OAuth 2.0 + PKCE
    OAuth,
    /// Claude Code CLI 认证
    ClaudeCode,
    /// Anthropic Console OAuth
    Console,
    /// 只读推理 Token
    SetupToken,
    /// AWS Bedrock Claude
    Bedrock,
    /// 第三方中转服务
    Ccr,
}

impl Default for AuthType {
    fn default() -> Self {
        Self::OAuth
    }
}

impl std::fmt::Display for AuthType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthType::OAuth => write!(f, "oauth"),
            AuthType::ClaudeCode => write!(f, "claude_code"),
            AuthType::Console => write!(f, "console"),
            AuthType::SetupToken => write!(f, "setup_token"),
            AuthType::Bedrock => write!(f, "bedrock"),
            AuthType::Ccr => write!(f, "ccr"),
        }
    }
}

/// Claude OAuth 凭证
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClaudeCredentials {
    /// 凭证名称
    #[serde(default)]
    pub name: Option<String>,
    /// 认证类型
    #[serde(default)]
    pub auth_type: AuthType,
    /// Access Token
    pub access_token: Option<String>,
    /// Refresh Token
    pub refresh_token: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 过期时间 (RFC3339 格式)
    pub expire: Option<String>,
    /// 最后刷新时间
    pub last_refresh: Option<String>,
    /// 是否健康
    #[serde(default = "default_true")]
    pub is_healthy: bool,
    /// 使用次数
    #[serde(default)]
    pub usage_count: u64,
    /// 错误次数
    #[serde(default)]
    pub error_count: u64,
    /// 最后错误信息
    #[serde(default)]
    pub last_error: Option<String>,

    // Bedrock 特有字段
    /// AWS Access Key ID
    pub access_key_id: Option<String>,
    /// AWS Secret Access Key
    pub secret_access_key: Option<String>,
    /// AWS Session Token
    pub session_token: Option<String>,
    /// AWS Region
    #[serde(default = "default_region")]
    pub region: Option<String>,

    // CCR 特有字段
    /// API Key
    pub api_key: Option<String>,
    /// Base URL
    pub base_url: Option<String>,

    // Console 特有字段
    /// Organization ID
    pub organization_id: Option<String>,
    /// Organization Name
    pub organization_name: Option<String>,
}

fn default_region() -> Option<String> {
    Some("us-east-1".to_string())
}

fn default_true() -> bool {
    true
}

impl Default for ClaudeCredentials {
    fn default() -> Self {
        Self {
            name: None,
            auth_type: AuthType::OAuth,
            access_token: None,
            refresh_token: None,
            email: None,
            expire: None,
            last_refresh: None,
            is_healthy: true,
            usage_count: 0,
            error_count: 0,
            last_error: None,
            access_key_id: None,
            secret_access_key: None,
            session_token: None,
            region: default_region(),
            api_key: None,
            base_url: None,
            organization_id: None,
            organization_name: None,
        }
    }
}

/// 获取的凭证
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcquiredCredential {
    /// 凭证 ID
    pub id: String,
    /// 凭证名称
    #[serde(default)]
    pub name: Option<String>,
    /// 认证方式
    pub auth_type: String,
    /// Base URL（如果有）
    #[serde(default)]
    pub base_url: Option<String>,
    /// 请求头（Key-Value 对）
    #[serde(default)]
    pub headers: HashMap<String, String>,
    /// 额外元数据
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// 凭证验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// 是否有效
    pub valid: bool,
    /// 消息
    #[serde(default)]
    pub message: Option<String>,
    /// 额外信息
    #[serde(default)]
    pub details: HashMap<String, serde_json::Value>,
}

/// OAuth 参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthParams {
    /// 授权 URL
    pub auth_url: String,
    /// Code Verifier (PKCE)
    pub code_verifier: String,
    /// State
    pub state: String,
    /// Code Challenge
    pub code_challenge: String,
}

/// OAuth Token 响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokens {
    /// Access Token
    pub access_token: String,
    /// Refresh Token
    pub refresh_token: Option<String>,
    /// 过期时间
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 邮箱
    pub email: Option<String>,
}
