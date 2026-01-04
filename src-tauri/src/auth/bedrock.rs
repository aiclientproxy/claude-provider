//! AWS Bedrock 认证模块
//!
//! 实现 AWS Bedrock Claude 模型的认证和调用

#![allow(dead_code)]

use anyhow::Result;
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Bedrock 凭证
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BedrockCredentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: Option<String>,
    pub region: String,
    pub default_model: Option<String>,
}

/// Bedrock 模型映射
pub const BEDROCK_MODEL_MAP: &[(&str, &str)] = &[
    (
        "claude-opus-4-20250514",
        "us.anthropic.claude-opus-4-20250514-v1:0",
    ),
    (
        "claude-opus-4-5-20251101",
        "us.anthropic.claude-opus-4-5-20251101-v1:0",
    ),
    (
        "claude-sonnet-4-20250514",
        "us.anthropic.claude-sonnet-4-20250514-v1:0",
    ),
    (
        "claude-sonnet-4-5-20250929",
        "us.anthropic.claude-sonnet-4-5-20250929-v1:0",
    ),
    (
        "claude-haiku-3-5-20241022",
        "us.anthropic.claude-haiku-3-5-20241022-v1:0",
    ),
    (
        "claude-3-5-sonnet-20241022",
        "us.anthropic.claude-3-5-sonnet-20241022-v2:0",
    ),
];

/// 将 Anthropic 模型名映射到 Bedrock 模型 ID
pub fn map_to_bedrock_model(model: &str) -> String {
    for (anthropic_model, bedrock_model) in BEDROCK_MODEL_MAP {
        if model == *anthropic_model {
            return bedrock_model.to_string();
        }
    }
    // 默认映射规则
    format!("us.anthropic.{}-v1:0", model)
}

/// AWS 签名 V4
pub struct AwsSignature {
    pub authorization: String,
    pub x_amz_date: String,
    pub x_amz_security_token: Option<String>,
}

/// 生成 AWS 签名 V4
pub fn sign_aws_request(
    method: &str,
    url: &str,
    credentials: &BedrockCredentials,
    body: &[u8],
) -> Result<AwsSignature> {
    let now = Utc::now();
    let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
    let date_stamp = now.format("%Y%m%d").to_string();

    let parsed_url = reqwest::Url::parse(url)?;
    let host = parsed_url.host_str().unwrap_or("");
    let canonical_uri = parsed_url.path();
    let canonical_querystring = parsed_url.query().unwrap_or("");

    // 计算 payload hash
    let payload_hash = hex::encode(Sha256::digest(body));

    // 构建 canonical headers
    let canonical_headers = format!(
        "host:{}\nx-amz-date:{}\n",
        host, amz_date
    );
    let signed_headers = "host;x-amz-date";

    // 构建 canonical request
    let canonical_request = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        method,
        canonical_uri,
        canonical_querystring,
        canonical_headers,
        signed_headers,
        payload_hash
    );

    // 计算 canonical request hash
    let canonical_request_hash = hex::encode(Sha256::digest(canonical_request.as_bytes()));

    // 构建 string to sign
    let algorithm = "AWS4-HMAC-SHA256";
    let credential_scope = format!("{}/{}/bedrock/aws4_request", date_stamp, credentials.region);
    let string_to_sign = format!(
        "{}\n{}\n{}\n{}",
        algorithm, amz_date, credential_scope, canonical_request_hash
    );

    // 计算签名
    let signing_key = get_signature_key(
        &credentials.secret_access_key,
        &date_stamp,
        &credentials.region,
        "bedrock",
    );
    let signature = hex::encode(hmac_sha256(&signing_key, string_to_sign.as_bytes()));

    // 构建 authorization header
    let authorization = format!(
        "{} Credential={}/{}, SignedHeaders={}, Signature={}",
        algorithm, credentials.access_key_id, credential_scope, signed_headers, signature
    );

    Ok(AwsSignature {
        authorization,
        x_amz_date: amz_date,
        x_amz_security_token: credentials.session_token.clone(),
    })
}

/// 生成签名密钥
fn get_signature_key(key: &str, date_stamp: &str, region: &str, service: &str) -> Vec<u8> {
    let k_date = hmac_sha256(format!("AWS4{}", key).as_bytes(), date_stamp.as_bytes());
    let k_region = hmac_sha256(&k_date, region.as_bytes());
    let k_service = hmac_sha256(&k_region, service.as_bytes());
    hmac_sha256(&k_service, b"aws4_request")
}

/// HMAC-SHA256
fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    use sha2::Sha256;
    use std::iter::repeat;

    let block_size = 64;
    let mut key = key.to_vec();

    if key.len() > block_size {
        key = Sha256::digest(&key).to_vec();
    }

    if key.len() < block_size {
        key.extend(repeat(0u8).take(block_size - key.len()));
    }

    let mut i_key_pad: Vec<u8> = key.iter().map(|&b| b ^ 0x36).collect();
    let mut o_key_pad: Vec<u8> = key.iter().map(|&b| b ^ 0x5c).collect();

    i_key_pad.extend_from_slice(data);
    let inner_hash = Sha256::digest(&i_key_pad);

    o_key_pad.extend_from_slice(&inner_hash);
    Sha256::digest(&o_key_pad).to_vec()
}

/// 验证 Bedrock 凭证
pub async fn validate_bedrock_credentials(credentials: &BedrockCredentials) -> Result<bool> {
    let client = Client::builder()
        .connect_timeout(std::time::Duration::from_secs(10))
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let url = format!(
        "https://bedrock.{}.amazonaws.com/foundation-models",
        credentials.region
    );

    let signature = sign_aws_request("GET", &url, credentials, &[])?;

    let mut request = client
        .get(&url)
        .header("Authorization", &signature.authorization)
        .header("x-amz-date", &signature.x_amz_date)
        .header("Host", format!("bedrock.{}.amazonaws.com", credentials.region));

    if let Some(token) = &signature.x_amz_security_token {
        request = request.header("x-amz-security-token", token);
    }

    let response = request.send().await?;

    Ok(response.status().is_success())
}

/// 构建 Bedrock API URL
pub fn build_bedrock_url(region: &str, model_id: &str) -> String {
    format!(
        "https://bedrock-runtime.{}.amazonaws.com/model/{}/invoke-with-response-stream",
        region, model_id
    )
}

/// hex 编码
mod hex {
    pub fn encode(data: impl AsRef<[u8]>) -> String {
        data.as_ref()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_to_bedrock_model() {
        assert_eq!(
            map_to_bedrock_model("claude-opus-4-5-20251101"),
            "us.anthropic.claude-opus-4-5-20251101-v1:0"
        );
        assert_eq!(
            map_to_bedrock_model("claude-sonnet-4-5-20250929"),
            "us.anthropic.claude-sonnet-4-5-20250929-v1:0"
        );
    }

    #[test]
    fn test_build_bedrock_url() {
        let url = build_bedrock_url("us-east-1", "us.anthropic.claude-opus-4-5-20251101-v1:0");
        assert!(url.contains("bedrock-runtime.us-east-1.amazonaws.com"));
        assert!(url.contains("invoke-with-response-stream"));
    }
}
