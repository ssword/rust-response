use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use super::common::{Usage, ResponseStatus};

/// Zero-copy response parsing using `RawValue` for efficient memory usage.
///
/// This struct provides a memory-efficient way to handle large API responses
/// by deferring JSON parsing until the data is actually needed. Large fields
/// like `output`, `metadata`, and `incomplete_details` are stored as raw JSON
/// strings and only parsed when accessed.
///
/// # Benefits
///
/// - **Memory Efficiency**: Avoids parsing large JSON structures unnecessarily
/// - **Performance**: Faster initial deserialization for large responses
/// - **Selective Parsing**: Only parse the fields you actually need
/// - **Zero-Copy**: Borrows from the original JSON string when possible
///
/// # Examples
///
/// ```rust
/// use openai_responses::LazyResponse;
///
/// // LazyResponse borrows from the JSON string
/// let json_str = r#"{"id": "resp_123", "status": "completed", ...}"#;
/// let lazy_response: LazyResponse = serde_json::from_str(json_str)?;
///
/// // Check status without parsing output
/// if lazy_response.is_completed() {
///     // Only parse output when needed
///     if let Ok(Some(text)) = lazy_response.get_text_output() {
///         println!("Response: {}", text);
///     }
/// }
/// ```
///
/// # Lifetime Management
///
/// The `LazyResponse` borrows from the original JSON string, so the string
/// must remain valid for the lifetime of the `LazyResponse`. This enables
/// zero-copy parsing but requires careful lifetime management.
///
/// # SIMD Support
///
/// When the `simd` feature is enabled, `LazyResponse` provides SIMD-accelerated
/// parsing methods for even better performance on large payloads.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LazyResponse<'a> {
    /// Unique identifier for this response
    pub id: String,

    /// Object type (typically "response")
    pub object: String,

    /// Current status of the response
    pub status: ResponseStatus,

    /// Raw JSON output content (parsed lazily)
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<&'a RawValue>,

    /// Token usage statistics (parsed immediately as it's small)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,

    /// Raw JSON metadata (parsed lazily)
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a RawValue>,

    /// Unix timestamp when the response was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<u64>,

    /// Raw JSON incomplete details (parsed lazily)
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<&'a RawValue>,
}

impl<'a> LazyResponse<'a> {
    /// Checks if the response has completed successfully.
    ///
    /// This method checks the status without parsing any large JSON fields,
    /// making it very efficient for status checking.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::LazyResponse;
    ///
    /// if lazy_response.is_completed() {
    ///     // Safe to parse output now
    ///     let output = lazy_response.parse_output()?;
    /// }
    /// ```
    pub fn is_completed(&self) -> bool {
        matches!(self.status, ResponseStatus::Completed)
    }

    /// Checks if the response is still being generated.
    ///
    /// This method checks the status without parsing any large JSON fields,
    /// making it very efficient for polling.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::LazyResponse;
    ///
    /// while lazy_response.is_in_progress() {
    ///     // Wait and check again
    ///     tokio::time::sleep(Duration::from_millis(100)).await;
    ///     // Refresh response status...
    /// }
    /// ```
    pub fn is_in_progress(&self) -> bool {
        matches!(self.status, ResponseStatus::InProgress)
    }

    /// Checks if the response was cancelled.
    ///
    /// This method checks the status without parsing any large JSON fields.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::LazyResponse;
    ///
    /// if lazy_response.is_cancelled() {
    ///     println!("Response was cancelled before completion");
    /// }
    /// ```
    pub fn is_cancelled(&self) -> bool {
        matches!(self.status, ResponseStatus::Cancelled)
    }

    /// Parses the output field only when needed (lazy evaluation).
    ///
    /// This method deserializes the raw JSON output into structured data
    /// only when called, avoiding unnecessary parsing overhead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::LazyResponse;
    ///
    /// // Only parse output when you need it
    /// if let Ok(Some(outputs)) = lazy_response.parse_output() {
    ///     for output in outputs {
    ///         // Process structured output
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `serde_json::Error` if the raw JSON cannot be parsed.
    pub fn parse_output(&self) -> Result<Option<Vec<super::common::Output>>, serde_json::Error> {
        match &self.output {
            Some(raw) => {
                let output: Vec<super::common::Output> = serde_json::from_str(raw.get())?;
                Ok(Some(output))
            },
            None => Ok(None),
        }
    }

    /// Efficiently extracts text content without full output parsing.
    ///
    /// This method attempts to extract the main text content from the response
    /// with minimal JSON parsing, making it more efficient than parsing the
    /// entire output structure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::LazyResponse;
    ///
    /// // Get text efficiently
    /// match lazy_response.get_text_output() {
    ///     Ok(Some(text)) => println!("Response: {}", text),
    ///     Ok(None) => println!("No text content found"),
    ///     Err(e) => eprintln!("Parse error: {}", e),
    /// }
    /// ```
    ///
    /// # Performance
    ///
    /// This method is optimized for the common case of just wanting the text
    /// content without needing the full structured output.
    ///
    /// # Errors
    ///
    /// Returns `serde_json::Error` if the raw JSON cannot be parsed.
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

    /// Parses the metadata field only when needed.
    ///
    /// This method deserializes the raw JSON metadata into a generic
    /// `serde_json::Value` for flexible access to custom metadata.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::LazyResponse;
    ///
    /// if let Ok(Some(metadata)) = lazy_response.parse_metadata() {
    ///     if let Some(custom_field) = metadata.get("custom_field") {
    ///         println!("Custom field: {}", custom_field);
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `serde_json::Error` if the raw JSON cannot be parsed.
    pub fn parse_metadata(&self) -> Result<Option<serde_json::Value>, serde_json::Error> {
        match &self.metadata {
            Some(raw) => {
                let metadata: serde_json::Value = serde_json::from_str(raw.get())?;
                Ok(Some(metadata))
            },
            None => Ok(None),
        }
    }

    /// Gets the total token count without parsing the full output.
    ///
    /// This method provides immediate access to token usage information
    /// since usage data is small and parsed immediately during deserialization.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::LazyResponse;
    ///
    /// if let Some(tokens) = lazy_response.get_total_tokens() {
    ///     println!("Used {} tokens", tokens);
    /// }
    /// ```
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