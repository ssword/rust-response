use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Usage {
    pub total_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResponseStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "cancelled")]
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image_url")]
    ImageUrl,
    #[serde(rename = "refusal")]
    Refusal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: ContentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub role: String,
    pub content: Vec<Content>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Output {
    #[serde(rename = "type")]
    pub output_type: String,
    pub content: Vec<Content>,
}