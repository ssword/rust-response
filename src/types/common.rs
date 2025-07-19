use serde::{Deserialize, Serialize};

/// Token usage statistics for API requests and responses.
///
/// This struct tracks how many tokens were consumed during the API call,
/// broken down by input (prompt) and output (completion) tokens.
///
/// # Examples
///
/// ```rust
/// use openai_responses::Usage;
///
/// // Check total token usage
/// if let Some(usage) = &response.usage {
///     println!("Total tokens used: {}", usage.total_tokens);
///
///     if let Some(prompt_tokens) = usage.prompt_tokens {
///         println!("Prompt tokens: {}", prompt_tokens);
///     }
///
///     if let Some(completion_tokens) = usage.completion_tokens {
///         println!("Completion tokens: {}", completion_tokens);
///     }
/// }
/// ```
///
/// # Token Counting
///
/// - **Total tokens**: Sum of prompt and completion tokens
/// - **Prompt tokens**: Tokens in the input/prompt
/// - **Completion tokens**: Tokens in the generated response
///
/// Token counts are used for billing and rate limiting purposes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Usage {
    /// Total number of tokens used (prompt + completion)
    pub total_tokens: u32,

    /// Number of tokens in the input/prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<u32>,

    /// Number of tokens in the generated completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<u32>,
}

/// Detailed error information from the API.
///
/// This struct contains comprehensive information about errors returned
/// by the OpenAI API, including the error message, type, and optional
/// additional context.
///
/// # Examples
///
/// ```rust
/// use openai_responses::ErrorDetail;
///
/// // Handle API errors
/// match api_result {
///     Ok(response) => { /* handle success */ },
///     Err(error_response) => {
///         let error = &error_response.error;
///         println!("Error: {} ({})", error.message, error.error_type);
///
///         if let Some(code) = &error.code {
///             println!("Error code: {}", code);
///         }
///
///         if let Some(param) = &error.param {
///             println!("Invalid parameter: {}", param);
///         }
///     }
/// }
/// ```
///
/// # Common Error Types
///
/// - `invalid_request_error` - Invalid request parameters
/// - `authentication_error` - Invalid API key
/// - `permission_error` - Insufficient permissions
/// - `rate_limit_error` - Rate limit exceeded
/// - `server_error` - Internal server error
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorDetail {
    /// Human-readable error message
    pub message: String,

    /// Error type classification
    #[serde(rename = "type")]
    pub error_type: String,

    /// Optional error code for programmatic handling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Optional parameter name that caused the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

/// API error response wrapper.
///
/// This struct wraps the error details in the format returned by the API.
/// All API errors are returned in this standardized format.
///
/// # Examples
///
/// ```rust
/// use openai_responses::ErrorResponse;
///
/// // Parse error response from JSON
/// let error_json = r#"{"error": {"message": "Invalid API key", "type": "authentication_error"}}"#;
/// let error_response: ErrorResponse = serde_json::from_str(error_json)?;
///
/// println!("Error: {}", error_response.error.message);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorResponse {
    /// The error details
    pub error: ErrorDetail,
}

/// Status of a response generation process.
///
/// This enum represents the current state of a response as it progresses
/// through the generation pipeline. Responses start as `InProgress` and
/// transition to a final state.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{ResponseStatus, Response};
///
/// match response.status {
///     ResponseStatus::InProgress => {
///         println!("Response is still being generated...");
///         // Poll again later
///     },
///     ResponseStatus::Completed => {
///         println!("Response is ready!");
///         // Process the output
///     },
///     ResponseStatus::Incomplete => {
///         println!("Response was cut off");
///         // Check incomplete_details for reason
///     },
///     ResponseStatus::Cancelled => {
///         println!("Response was cancelled");
///     },
/// }
/// ```
///
/// # State Transitions
///
/// ```text
/// InProgress → Completed (successful completion)
/// InProgress → Incomplete (cut off due to limits)
/// InProgress → Cancelled (manually cancelled)
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResponseStatus {
    /// Response is currently being generated
    #[serde(rename = "in_progress")]
    InProgress,

    /// Response generation completed successfully
    #[serde(rename = "completed")]
    Completed,

    /// Response was cut off before completion
    #[serde(rename = "incomplete")]
    Incomplete,

    /// Response generation was cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
}

/// Type of content in a message or output.
///
/// This enum specifies the format and type of content, enabling
/// multimodal responses that can include text, images, and refusals.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{ContentType, Content};
///
/// // Process different content types
/// for content in &output.content {
///     match content.content_type {
///         ContentType::Text => {
///             if let Some(text) = &content.text {
///                 println!("Text: {}", text);
///             }
///         },
///         ContentType::ImageUrl => {
///             if let Some(url) = &content.image_url {
///                 println!("Image: {}", url);
///             }
///         },
///         ContentType::Refusal => {
///             println!("Model refused to respond");
///         },
///     }
/// }
/// ```
///
/// # Content Types
///
/// - **Text**: Plain text content
/// - **ImageUrl**: URL to an image resource
/// - **Refusal**: Model declined to provide content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    /// Plain text content
    #[serde(rename = "text")]
    Text,

    /// Image URL content
    #[serde(rename = "image_url")]
    ImageUrl,

    /// Model refusal to provide content
    #[serde(rename = "refusal")]
    Refusal,
}

/// Individual piece of content within a message or output.
///
/// This struct represents a single content item, which can be text,
/// an image URL, or a refusal. Multiple content items can be combined
/// to create multimodal messages.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{Content, ContentType};
///
/// // Text content
/// let text_content = Content {
///     content_type: ContentType::Text,
///     text: Some("Hello, world!".to_string()),
///     image_url: None,
/// };
///
/// // Image content
/// let image_content = Content {
///     content_type: ContentType::ImageUrl,
///     text: None,
///     image_url: Some("https://example.com/image.jpg".to_string()),
/// };
///
/// // Extract text from content
/// if let Some(text) = &content.text {
///     println!("Content text: {}", text);
/// }
/// ```
///
/// # Multimodal Support
///
/// Content items can be combined to create rich, multimodal interactions
/// that include both text and images in a single message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Content {
    /// Type of content (text, image_url, refusal)
    #[serde(rename = "type")]
    pub content_type: ContentType,

    /// Text content (present when content_type is Text)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Image URL (present when content_type is ImageUrl)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

/// Message structure for conversation context.
///
/// This struct represents a single message in a conversation, containing
/// the role of the speaker and the content of the message.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{Message, Content, ContentType};
///
/// // Create a user message
/// let user_message = Message {
///     role: "user".to_string(),
///     content: vec![Content {
///         content_type: ContentType::Text,
///         text: Some("What is the capital of France?".to_string()),
///         image_url: None,
///     }],
/// };
///
/// // Create an assistant message
/// let assistant_message = Message {
///     role: "assistant".to_string(),
///     content: vec![Content {
///         content_type: ContentType::Text,
///         text: Some("The capital of France is Paris.".to_string()),
///         image_url: None,
///     }],
/// };
/// ```
///
/// # Common Roles
///
/// - `"user"` - Messages from the user
/// - `"assistant"` - Messages from the AI assistant
/// - `"system"` - System instructions and context
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    /// Role of the message sender (user, assistant, system)
    pub role: String,

    /// Content items that make up the message
    pub content: Vec<Content>,
}

/// Output generated by the model.
///
/// This struct represents the structured output from the model, containing
/// the type of output and the actual content items.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{Output, Content, ContentType};
///
/// // Process model output
/// for output in &response.output {
///     println!("Output type: {}", output.output_type);
///
///     for content in &output.content {
///         match content.content_type {
///             ContentType::Text => {
///                 if let Some(text) = &content.text {
///                     println!("Generated text: {}", text);
///                 }
///             },
///             ContentType::ImageUrl => {
///                 if let Some(url) = &content.image_url {
///                     println!("Generated image: {}", url);
///                 }
///             },
///             ContentType::Refusal => {
///                 println!("Model refused to generate content");
///             },
///         }
///     }
/// }
/// ```
///
/// # Output Types
///
/// The `output_type` field indicates the kind of output generated,
/// such as "text", "image", or other model-specific types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Output {
    /// Type of output generated
    #[serde(rename = "type")]
    pub output_type: String,

    /// Content items that make up the output
    pub content: Vec<Content>,
}