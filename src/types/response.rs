use serde::{Deserialize, Serialize};
use super::common::{Output, Usage, ResponseStatus};

/// Response from the OpenAI Responses API.
/// 
/// This struct represents a complete response from the API, including
/// the generated content, usage statistics, and metadata.
/// 
/// # Examples
/// 
/// ```rust
/// use openai_responses::{Response, ResponseStatus};
/// 
/// // Check if response is completed
/// if response.is_completed() {
///     if let Some(text) = response.get_text_output() {
///         println!("Response: {}", text);
///     }
/// }
/// 
/// // Get token usage
/// if let Some(tokens) = response.get_total_tokens() {
///     println!("Used {} tokens", tokens);
/// }
/// ```
/// 
/// # Status Checking
/// 
/// Use the convenience methods to check response status:
/// - [`is_completed()`](Response::is_completed) - Response is finished
/// - [`is_in_progress()`](Response::is_in_progress) - Response is still generating
/// - [`is_cancelled()`](Response::is_cancelled) - Response was cancelled
/// 
/// # Content Extraction
/// 
/// Use [`get_text_output()`](Response::get_text_output) to extract the main
/// text content from the response, or access the [`output`](Response::output)
/// field directly for more detailed content analysis.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Response {
    /// Unique identifier for this response
    pub id: String,
    
    /// Object type (typically "response")
    pub object: String,
    
    /// Current status of the response
    pub status: ResponseStatus,
    
    /// Generated output content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<Output>>,
    
    /// Token usage statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    
    /// Custom metadata attached to the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    
    /// Unix timestamp when the response was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<u64>,
    
    /// Details about incomplete responses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<IncompleteDetails>,
}

/// Details about why a response is incomplete.
/// 
/// This struct provides information about responses that didn't complete
/// successfully, including the reason for incompletion.
/// 
/// # Examples
/// 
/// ```rust
/// use openai_responses::{Response, IncompleteDetails};
/// 
/// if let Some(details) = &response.incomplete_details {
///     if let Some(reason) = &details.reason {
///         println!("Response incomplete: {}", reason);
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IncompleteDetails {
    /// Reason why the response is incomplete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Response from a delete operation.
/// 
/// This struct represents the result of deleting a response,
/// confirming whether the deletion was successful.
/// 
/// # Examples
/// 
/// ```rust
/// use openai_responses::DeletedResponse;
/// 
/// let deleted = client.delete_response("resp_123").await?;
/// if deleted.deleted {
///     println!("Response {} was successfully deleted", deleted.id);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeletedResponse {
    /// ID of the deleted response
    pub id: String,
    
    /// Object type (typically "response.deleted")
    pub object: String,
    
    /// Whether the deletion was successful
    pub deleted: bool,
}

/// List of responses with pagination information.
/// 
/// This struct represents a paginated list of responses returned
/// by the list responses endpoint.
/// 
/// # Examples
/// 
/// ```rust
/// use openai_responses::ResponseList;
/// 
/// let response_list = client.list_responses(Some(10), None, None).await?;
/// 
/// println!("Found {} responses", response_list.data.len());
/// 
/// for response in response_list.data {
///     println!("Response {}: {:?}", response.id, response.status);
/// }
/// 
/// if response_list.has_more == Some(true) {
///     println!("More responses available");
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseList {
    /// Object type (typically "list")
    pub object: String,
    
    /// Array of response objects
    pub data: Vec<Response>,
    
    /// ID of the first response in the list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    
    /// ID of the last response in the list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    
    /// Whether there are more responses available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl Response {
    /// Checks if the response has completed successfully.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use openai_responses::Response;
    /// 
    /// if response.is_completed() {
    ///     println!("Response is ready!");
    /// }
    /// ```
    pub fn is_completed(&self) -> bool {
        matches!(self.status, ResponseStatus::Completed)
    }

    /// Checks if the response is still being generated.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use openai_responses::Response;
    /// 
    /// if response.is_in_progress() {
    ///     println!("Still generating...");
    /// }
    /// ```
    pub fn is_in_progress(&self) -> bool {
        matches!(self.status, ResponseStatus::InProgress)
    }

    /// Checks if the response was cancelled.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use openai_responses::Response;
    /// 
    /// if response.is_cancelled() {
    ///     println!("Response was cancelled");
    /// }
    /// ```
    pub fn is_cancelled(&self) -> bool {
        matches!(self.status, ResponseStatus::Cancelled)
    }

    /// Extracts the main text content from the response.
    /// 
    /// This method searches through the output for the first text content
    /// and returns it. Returns `None` if no text content is found.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use openai_responses::Response;
    /// 
    /// if let Some(text) = response.get_text_output() {
    ///     println!("Response text: {}", text);
    /// } else {
    ///     println!("No text content found");
    /// }
    /// ```
    pub fn get_text_output(&self) -> Option<String> {
        self.output.as_ref().and_then(|outputs| {
            outputs.iter().find_map(|output| {
                output.content.iter().find_map(|content| {
                    content.text.clone()
                })
            })
        })
    }

    /// Gets the total number of tokens used in this response.
    /// 
    /// Returns the total token count from the usage statistics,
    /// or `None` if usage information is not available.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use openai_responses::Response;
    /// 
    /// if let Some(tokens) = response.get_total_tokens() {
    ///     println!("Used {} tokens", tokens);
    /// }
    /// ```
    pub fn get_total_tokens(&self) -> Option<u32> {
        self.usage.as_ref().map(|usage| usage.total_tokens)
    }
}

/// Streaming response for real-time updates.
///
/// This struct represents a streaming response that provides
/// incremental updates as the response is being generated.
///
/// # Examples
///
/// ```rust
/// use openai_responses::StreamResponse;
///
/// // Process streaming response
/// if let Some(delta) = &stream_response.delta {
///     // Handle incremental content
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StreamResponse {
    /// Unique identifier for this response
    pub id: String,

    /// Object type (typically "response.stream")
    pub object: String,

    /// Current status of the response
    pub status: ResponseStatus,

    /// Complete output content (may be partial during streaming)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<Output>>,

    /// Token usage statistics (updated incrementally)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,

    /// Incremental content delta for this update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<Output>,
}
