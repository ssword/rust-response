use serde::{Deserialize, Serialize};
use super::common::{Output, Usage, ResponseStatus};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub status: ResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<Output>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<IncompleteDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IncompleteDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeletedResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseList {
    pub object: String,
    pub data: Vec<Response>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl Response {
    pub fn is_completed(&self) -> bool {
        matches!(self.status, ResponseStatus::Completed)
    }

    pub fn is_in_progress(&self) -> bool {
        matches!(self.status, ResponseStatus::InProgress)
    }

    pub fn is_cancelled(&self) -> bool {
        matches!(self.status, ResponseStatus::Cancelled)
    }

    pub fn get_text_output(&self) -> Option<String> {
        self.output.as_ref().and_then(|outputs| {
            outputs.iter().find_map(|output| {
                output.content.iter().find_map(|content| {
                    content.text.clone()
                })
            })
        })
    }

    pub fn get_total_tokens(&self) -> Option<u32> {
        self.usage.as_ref().map(|usage| usage.total_tokens)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StreamResponse {
    pub id: String,
    pub object: String,
    pub status: ResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<Output>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<Output>,
}