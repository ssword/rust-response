use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use super::common::{Usage, ResponseStatus};

/// Zero-copy response parsing using RawValue for large JSON fields
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LazyResponse<'a> {
    pub id: String,
    pub object: String,
    pub status: ResponseStatus,
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<&'a RawValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a RawValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<u64>,
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<&'a RawValue>,
}

impl<'a> LazyResponse<'a> {
    /// Check if response is completed without parsing output
    pub fn is_completed(&self) -> bool {
        matches!(self.status, ResponseStatus::Completed)
    }

    /// Check if response is in progress without parsing output
    pub fn is_in_progress(&self) -> bool {
        matches!(self.status, ResponseStatus::InProgress)
    }

    /// Check if response is cancelled without parsing output
    pub fn is_cancelled(&self) -> bool {
        matches!(self.status, ResponseStatus::Cancelled)
    }

    /// Parse output only when needed - lazy evaluation
    pub fn parse_output(&self) -> Result<Option<Vec<super::common::Output>>, serde_json::Error> {
        match &self.output {
            Some(raw) => {
                let output: Vec<super::common::Output> = serde_json::from_str(raw.get())?;
                Ok(Some(output))
            },
            None => Ok(None),
        }
    }

    /// Get text output efficiently without full parsing
    pub fn get_text_output(&self) -> Result<Option<String>, serde_json::Error> {
        match &self.output {
            Some(raw) => {
                // Try to extract text without full deserialization
                let json_str = raw.get();
                if let Ok(outputs) = serde_json::from_str::<Vec<super::common::Output>>(json_str) {
                    for output in outputs {
                        for content in output.content {
                            if let Some(text) = content.text {
                                return Ok(Some(text));
                            }
                        }
                    }
                }
                Ok(None)
            },
            None => Ok(None),
        }
    }

    /// Parse metadata only when needed
    pub fn parse_metadata(&self) -> Result<Option<serde_json::Value>, serde_json::Error> {
        match &self.metadata {
            Some(raw) => {
                let metadata: serde_json::Value = serde_json::from_str(raw.get())?;
                Ok(Some(metadata))
            },
            None => Ok(None),
        }
    }

    /// Get token count without parsing full output
    pub fn get_total_tokens(&self) -> Option<u32> {
        self.usage.as_ref().map(|usage| usage.total_tokens)
    }
}

#[cfg(feature = "simd")]
impl<'a> LazyResponse<'a> {
    /// Fast SIMD JSON parsing when available
    pub fn parse_output_simd(&self, buffer: &mut [u8]) -> Result<Option<Vec<super::common::Output>>, Box<dyn std::error::Error>> {
        match &self.output {
            Some(raw) => {
                let json_bytes = raw.get().as_bytes();
                if json_bytes.len() > buffer.len() {
                    return Err("Buffer too small for SIMD parsing".into());
                }
                buffer[..json_bytes.len()].copy_from_slice(json_bytes);
                let output: Vec<super::common::Output> = simd_json::from_slice(&mut buffer[..json_bytes.len()])?;
                Ok(Some(output))
            },
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_lazy_response_basic() {
        let json_str = r#"{
            "id": "resp_123",
            "object": "response",
            "status": "completed",
            "output": [{"type": "message", "content": [{"type": "text", "text": "Hello"}]}],
            "usage": {"total_tokens": 10}
        }"#;

        let lazy_response: LazyResponse = serde_json::from_str(json_str).unwrap();
        assert_eq!(lazy_response.id, "resp_123");
        assert!(lazy_response.is_completed());
        assert_eq!(lazy_response.get_total_tokens(), Some(10));
    }

    #[test]
    fn test_lazy_parsing() {
        let json_str = r#"{
            "id": "resp_123",
            "object": "response", 
            "status": "completed",
            "output": [{"type": "message", "content": [{"type": "text", "text": "Hello World"}]}]
        }"#;

        let lazy_response: LazyResponse = serde_json::from_str(json_str).unwrap();
        
        // Parse output only when needed
        let output = lazy_response.parse_output().unwrap();
        assert!(output.is_some());
        
        // Get text efficiently
        let text = lazy_response.get_text_output().unwrap();
        assert_eq!(text, Some("Hello World".to_string()));
    }
}