use serde::{Deserialize, Serialize};
use super::model::Model;
use super::enums::{ReasoningEffort, TruncationType, ResponseFormatType, ToolChoiceType, Modality};

/// Validation errors for request configuration.
///
/// This enum represents all possible validation errors that can occur
/// when building or validating a request against model capabilities
/// and parameter constraints.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{Model, ValidationError};
///
/// // This would cause a validation error
/// let model = Model::O1; // O1 doesn't support vision
/// // let error = ValidationError::VisionNotSupported(model);
/// ```
///
/// # Error Categories
///
/// * **Model Capability Errors**: When requesting features not supported by the model
/// * **Parameter Range Errors**: When parameters are outside valid ranges
///
/// All errors implement [`std::error::Error`] and can be used with `?` operator.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ValidationError {
    /// Model does not support reasoning capabilities.
    ///
    /// Occurs when trying to use reasoning configuration with models
    /// that don't support advanced reasoning (non-O1/O3/O4 models).
    #[error("Model {0} does not support reasoning capabilities")]
    ReasoningNotSupported(Model),

    /// Model does not support vision capabilities.
    ///
    /// Occurs when trying to use image modalities with models
    /// that don't support vision processing.
    #[error("Model {0} does not support vision capabilities")]
    VisionNotSupported(Model),

    /// Model does not support JSON mode.
    ///
    /// Occurs when trying to use JSON response format with models
    /// that don't support structured output.
    #[error("Model {0} does not support JSON mode")]
    JsonModeNotSupported(Model),

    /// Model does not support function calling.
    ///
    /// Occurs when trying to use tools/functions with models
    /// that don't support function calling.
    #[error("Model {0} does not support function calling")]
    FunctionCallingNotSupported(Model),

    /// Temperature parameter is outside valid range (0.0-2.0).
    ///
    /// Temperature controls randomness in the model's output.
    #[error("Temperature {0} must be between 0.0 and 2.0")]
    InvalidTemperature(f64),

    /// Top-p parameter is outside valid range (0.0-1.0).
    ///
    /// Top-p controls nucleus sampling for response generation.
    #[error("Top-p {0} must be between 0.0 and 1.0")]
    InvalidTopP(f64),

    /// Frequency penalty is outside valid range (-2.0-2.0).
    ///
    /// Frequency penalty reduces repetition of tokens.
    #[error("Frequency penalty {0} must be between -2.0 and 2.0")]
    InvalidFrequencyPenalty(f64),

    /// Presence penalty is outside valid range (-2.0-2.0).
    ///
    /// Presence penalty encourages talking about new topics.
    #[error("Presence penalty {0} must be between -2.0 and 2.0")]
    InvalidPresencePenalty(f64),
}

/// Request structure for creating OpenAI responses.
///
/// This struct represents a complete request to the OpenAI Responses API.
/// It includes all possible parameters and configurations, with optional
/// fields that are omitted from serialization when not set.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{Model, CreateResponseRequest};
///
/// // Simple request
/// let request = CreateResponseRequest::new(Model::Gpt4_1Nano, "Hello, world!");
///
/// // Request with additional configuration
/// let request = CreateResponseRequest::new(Model::Gpt4_1, "Analyze this image")
///     .with_temperature(0.7)
///     .with_max_tokens(500)
///     .with_instructions("Be detailed and thorough");
/// ```
///
/// # Validation
///
/// Requests should be validated against model capabilities using
/// [`validate_for_model()`](CreateResponseRequest::validate_for_model)
/// before sending to ensure compatibility.
///
/// # Builder Pattern
///
/// Use [`CreateResponseRequest::builder()`] for a more ergonomic
/// building experience with compile-time validation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponseRequest {
    /// The model to use for generating the response
    pub model: Model,

    /// The input text or prompt for the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,

    /// System instructions to guide the model's behavior
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Controls randomness in output (0.0-2.0, higher = more random)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// Whether to process the request in the background
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,

    /// Tools/functions available to the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    /// Nucleus sampling parameter (0.0-1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,

    /// Penalty for token frequency (-2.0-2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,

    /// Penalty for token presence (-2.0-2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,

    /// Random seed for deterministic output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,

    /// Custom metadata to attach to the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    /// Whether to store the conversation for future reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,

    /// Reasoning configuration for reasoning-capable models
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,

    /// Truncation strategy for conversation management
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<TruncationConfig>,

    /// Input modalities (text, image, audio)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modality>>,

    /// Desired response format (text, JSON, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,

    /// Whether to allow parallel tool calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// Strategy for tool selection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
}

/// Configuration for reasoning-capable models.
///
/// This struct controls how much computational effort reasoning models
/// put into solving problems. Only applicable to models that support
/// reasoning capabilities.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{ReasoningConfig, ReasoningEffort};
///
/// let config = ReasoningConfig {
///     effort: Some(ReasoningEffort::High),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReasoningConfig {
    /// The effort level for reasoning (low, medium, high)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<ReasoningEffort>,
}

/// Configuration for conversation truncation.
///
/// This struct controls how the API handles conversation context
/// when it exceeds the model's context window limits.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{TruncationConfig, TruncationType};
///
/// // Auto-truncate with no limit on turns
/// let config = TruncationConfig {
///     type_: Some(TruncationType::Auto),
///     last_turns: None,
/// };
///
/// // Auto-truncate keeping only last 5 turns
/// let config = TruncationConfig {
///     type_: Some(TruncationType::Auto),
///     last_turns: Some(5),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TruncationConfig {
    /// The truncation strategy to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<TruncationType>,

    /// Number of recent conversation turns to preserve
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_turns: Option<u32>,
}

/// Response format specification.
///
/// This enum defines the desired format for the model's response,
/// from free-form text to structured JSON with schemas.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{ResponseFormat, ResponseFormatType};
/// use serde_json::json;
///
/// // Plain text response
/// let text_format = ResponseFormat::Text {
///     type_: ResponseFormatType::Text
/// };
///
/// // JSON object response
/// let json_format = ResponseFormat::JsonObject {
///     type_: ResponseFormatType::JsonObject
/// };
///
/// // JSON with specific schema
/// let schema_format = ResponseFormat::JsonSchema {
///     type_: ResponseFormatType::JsonSchema,
///     json_schema: json!({
///         "type": "object",
///         "properties": {
///             "name": {"type": "string"},
///             "age": {"type": "number"}
///         }
///     }),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponseFormat {
    /// Plain text response format
    Text {
        /// Format type identifier
        type_: ResponseFormatType
    },
    /// JSON object response format
    JsonObject {
        /// Format type identifier
        type_: ResponseFormatType
    },
    /// JSON response conforming to a specific schema
    JsonSchema {
        /// Format type identifier
        type_: ResponseFormatType,
        /// JSON schema definition
        json_schema: serde_json::Value,
    },
}

/// Tool choice strategy for function calling.
///
/// This enum controls how the model should handle tool selection
/// when multiple tools are available.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{ToolChoice, ToolChoiceType};
///
/// // Let model decide
/// let auto_choice = ToolChoice::Auto;
///
/// // Force tool usage
/// let required_choice = ToolChoice::Required;
///
/// // Specify a particular tool
/// let specific_choice = ToolChoice::Specific {
///     type_: ToolChoiceType::Auto,
///     name: "web_search".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ToolChoice {
    /// Let the model decide when to use tools
    Auto,
    /// Require the model to use at least one tool
    Required,
    /// Don't use any tools
    None,
    /// Use a specific named tool
    Specific {
        /// Tool choice type
        type_: ToolChoiceType,
        /// Name of the specific tool to use
        name: String
    },
}

/// Tool/function definition for function calling.
///
/// This struct defines a tool that the model can call to perform
/// specific tasks like web searches, calculations, or API calls.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{Tool, ToolFunction};
/// use serde_json::json;
///
/// let tool = Tool {
///     tool_type: "function".to_string(),
///     function: Some(ToolFunction {
///         name: "get_weather".to_string(),
///         description: Some("Get current weather for a location".to_string()),
///         parameters: Some(json!({
///             "type": "object",
///             "properties": {
///                 "location": {
///                     "type": "string",
///                     "description": "City name"
///                 }
///             },
///             "required": ["location"]
///         })),
///     }),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tool {
    /// Type of tool (typically "function")
    #[serde(rename = "type")]
    pub tool_type: String,

    /// Function definition if this is a function tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<ToolFunction>,
}

/// Function definition for function calling tools.
///
/// This struct defines the interface for a callable function,
/// including its name, description, and parameter schema.
///
/// # Examples
///
/// ```rust
/// use openai_responses::ToolFunction;
/// use serde_json::json;
///
/// let function = ToolFunction {
///     name: "calculate_sum".to_string(),
///     description: Some("Add two numbers together".to_string()),
///     parameters: Some(json!({
///         "type": "object",
///         "properties": {
///             "a": {"type": "number"},
///             "b": {"type": "number"}
///         },
///         "required": ["a", "b"]
///     })),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolFunction {
    /// Name of the function
    pub name: String,

    /// Human-readable description of what the function does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// JSON schema defining the function's parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

impl CreateResponseRequest {
    /// Creates a new request with the specified model and input.
    ///
    /// This is the primary constructor for creating requests. All other
    /// parameters are optional and can be set using the builder methods.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::{Model, CreateResponseRequest};
    ///
    /// let request = CreateResponseRequest::new(
    ///     Model::Gpt4_1Nano,
    ///     "What is the capital of France?"
    /// );
    /// ```
    ///
    /// # Parameters
    ///
    /// * `model` - The [`Model`] to use for generating the response
    /// * `input` - The input text or prompt (can be any type that implements `Into<String>`)
    pub fn new(model: Model, input: impl Into<String>) -> Self {
        Self {
            model,
            input: Some(input.into()),
            instructions: None,
            temperature: None,
            max_tokens: None,
            background: None,
            tools: None,
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            seed: None,
            metadata: None,
            store: None,
            reasoning: None,
            truncation: None,
            modalities: None,
            response_format: None,
            parallel_tool_calls: None,
            tool_choice: None,
        }
    }

    pub fn with_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = Some(instructions.into());
        self
    }

    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn with_background(mut self, background: bool) -> Self {
        self.background = Some(background);
        self
    }

    pub fn with_tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }

    pub fn with_top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn with_frequency_penalty(mut self, penalty: f64) -> Self {
        self.frequency_penalty = Some(penalty);
        self
    }

    pub fn with_presence_penalty(mut self, penalty: f64) -> Self {
        self.presence_penalty = Some(penalty);
        self
    }

    pub fn with_seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn with_store(mut self, store: bool) -> Self {
        self.store = Some(store);
        self
    }

    pub fn with_reasoning(mut self, effort: ReasoningEffort) -> Self {
        self.reasoning = Some(ReasoningConfig {
            effort: Some(effort),
        });
        self
    }

    pub fn with_truncation(mut self, type_: TruncationType, last_turns: Option<u32>) -> Self {
        self.truncation = Some(TruncationConfig {
            type_: Some(type_),
            last_turns,
        });
        self
    }

    pub fn with_modalities(mut self, modalities: Vec<Modality>) -> Self {
        self.modalities = Some(modalities);
        self
    }

    pub fn with_response_format_text(mut self) -> Self {
        self.response_format = Some(ResponseFormat::Text {
            type_: ResponseFormatType::Text,
        });
        self
    }

    pub fn with_response_format_json(mut self) -> Self {
        self.response_format = Some(ResponseFormat::JsonObject {
            type_: ResponseFormatType::JsonObject,
        });
        self
    }

    pub fn with_response_format_schema(mut self, schema: serde_json::Value) -> Self {
        self.response_format = Some(ResponseFormat::JsonSchema {
            type_: ResponseFormatType::JsonSchema,
            json_schema: schema,
        });
        self
    }

    pub fn with_parallel_tool_calls(mut self, parallel: bool) -> Self {
        self.parallel_tool_calls = Some(parallel);
        self
    }

    pub fn with_tool_choice_auto(mut self) -> Self {
        self.tool_choice = Some(ToolChoice::Auto);
        self
    }

    pub fn with_tool_choice_required(mut self) -> Self {
        self.tool_choice = Some(ToolChoice::Required);
        self
    }

    pub fn with_tool_choice_none(mut self) -> Self {
        self.tool_choice = Some(ToolChoice::None);
        self
    }

    pub fn with_tool_choice_specific(mut self, name: impl Into<String>) -> Self {
        self.tool_choice = Some(ToolChoice::Specific {
            type_: ToolChoiceType::Auto,
            name: name.into(),
        });
        self
    }

    /// Validates the request configuration against the selected model capabilities.
    ///
    /// This method performs comprehensive validation to ensure that:
    /// - All requested features are supported by the selected model
    /// - All parameter values are within valid ranges
    /// - The configuration is internally consistent
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::{Model, CreateResponseRequest, ReasoningEffort};
    ///
    /// // This will pass validation
    /// let request = CreateResponseRequest::new(Model::O1, "Solve this problem")
    ///     .with_reasoning(ReasoningEffort::High);
    /// assert!(request.validate_for_model().is_ok());
    ///
    /// // This will fail validation (O1 doesn't support vision)
    /// let request = CreateResponseRequest::new(Model::O1, "Analyze this image")
    ///     .with_modalities(vec![Modality::Image]);
    /// assert!(request.validate_for_model().is_err());
    /// ```
    ///
    /// # Validation Rules
    ///
    /// ## Model Capabilities
    /// - Reasoning configuration requires reasoning-capable models
    /// - Image modalities require vision-capable models
    /// - JSON response format requires JSON-capable models
    /// - Tools require function-calling capable models
    ///
    /// ## Parameter Ranges
    /// - Temperature: 0.0 to 2.0
    /// - Top-p: 0.0 to 1.0
    /// - Frequency penalty: -2.0 to 2.0
    /// - Presence penalty: -2.0 to 2.0
    ///
    /// # Errors
    ///
    /// Returns [`ValidationError`] if any validation rule is violated.
    pub fn validate_for_model(&self) -> Result<(), ValidationError> {
        // Check reasoning configuration
        if self.reasoning.is_some() && !self.model.supports_reasoning() {
            return Err(ValidationError::ReasoningNotSupported(self.model));
        }

        // Check vision capabilities
        if let Some(modalities) = &self.modalities {
            if modalities.contains(&Modality::Image) && !self.model.supports_vision() {
                return Err(ValidationError::VisionNotSupported(self.model));
            }
        }

        // Check JSON mode support
        if let Some(ResponseFormat::JsonObject { .. }) = &self.response_format {
            if !self.model.supports_json_mode() {
                return Err(ValidationError::JsonModeNotSupported(self.model));
            }
        }

        // Check function calling support
        if self.tools.is_some() && !self.model.supports_function_calling() {
            return Err(ValidationError::FunctionCallingNotSupported(self.model));
        }

        // Validate temperature range
        if let Some(temp) = self.temperature {
            if !(0.0..=2.0).contains(&temp) {
                return Err(ValidationError::InvalidTemperature(temp));
            }
        }

        // Validate top_p range
        if let Some(top_p) = self.top_p {
            if !(0.0..=1.0).contains(&top_p) {
                return Err(ValidationError::InvalidTopP(top_p));
            }
        }

        // Validate frequency_penalty range
        if let Some(freq_penalty) = self.frequency_penalty {
            if !(-2.0..=2.0).contains(&freq_penalty) {
                return Err(ValidationError::InvalidFrequencyPenalty(freq_penalty));
            }
        }

        // Validate presence_penalty range
        if let Some(presence_penalty) = self.presence_penalty {
            if !(-2.0..=2.0).contains(&presence_penalty) {
                return Err(ValidationError::InvalidPresencePenalty(presence_penalty));
            }
        }

        Ok(())
    }

    /// Creates a builder for constructing requests with validation.
    ///
    /// The builder provides a fluent API for constructing requests with
    /// compile-time and runtime validation. It's the recommended way to
    /// build complex requests.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::{Model, CreateResponseRequest, ReasoningEffort};
    ///
    /// let request = CreateResponseRequest::builder(Model::Gpt4_1Nano, "Hello")
    ///     .temperature(0.7)?
    ///     .max_tokens(100)
    ///     .instructions("Be helpful and concise")
    ///     .build()?;
    /// ```
    ///
    /// # Parameters
    ///
    /// * `model` - The [`Model`] to use for the request
    /// * `input` - The input text or prompt
    ///
    /// # Returns
    ///
    /// A [`CreateResponseRequestBuilder`] for fluent request construction.
    pub fn builder(model: Model, input: impl Into<String>) -> CreateResponseRequestBuilder {
        CreateResponseRequestBuilder::new(model, input)
    }
}



/// Builder for [`CreateResponseRequest`] with validation and fluent API.
///
/// This builder provides a type-safe, ergonomic way to construct requests
/// with automatic validation of parameters and model capabilities.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{Model, CreateResponseRequest, ReasoningEffort, Modality};
///
/// // Simple request
/// let request = CreateResponseRequest::builder(Model::Gpt4_1Nano, "Hello")
///     .temperature(0.7)?
///     .build()?;
///
/// // Complex request with multiple features
/// let request = CreateResponseRequest::builder(Model::Gpt4_1, "Analyze this")
///     .instructions("Be thorough and detailed")?
///     .temperature(0.5)?
///     .max_tokens(1000)
///     .modalities(vec![Modality::Text, Modality::Image])?
///     .response_format_json()?
///     .build()?;
/// ```
///
/// # Validation
///
/// The builder performs validation at two levels:
/// 1. **Parameter validation**: Ensures values are within valid ranges
/// 2. **Model capability validation**: Ensures requested features are supported
///
/// Methods that can fail return `Result<Self, ValidationError>` for immediate
/// error handling, while safe methods return `Self` for continued chaining.
///
/// # Error Handling
///
/// ```rust
/// use openai_responses::{Model, CreateResponseRequest, ValidationError};
///
/// let result = CreateResponseRequest::builder(Model::O1, "Hello")
///     .temperature(3.0); // Invalid temperature
///
/// match result {
///     Ok(builder) => { /* continue building */ },
///     Err(ValidationError::InvalidTemperature(temp)) => {
///         println!("Temperature {} is invalid", temp);
///     },
///     Err(e) => println!("Other error: {}", e),
/// }
/// ```
#[derive(Debug)]
pub struct CreateResponseRequestBuilder {
    request: CreateResponseRequest,
}

impl CreateResponseRequestBuilder {
    pub fn new(model: Model, input: impl Into<String>) -> Self {
        Self {
            request: CreateResponseRequest::new(model, input),
        }
    }

    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.request = self.request.with_instructions(instructions);
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Result<Self, ValidationError> {
        if !(0.0..=2.0).contains(&temperature) {
            return Err(ValidationError::InvalidTemperature(temperature));
        }
        self.request = self.request.with_temperature(temperature);
        Ok(self)
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.request = self.request.with_max_tokens(max_tokens);
        self
    }

    pub fn background(mut self, background: bool) -> Self {
        self.request = self.request.with_background(background);
        self
    }

    pub fn top_p(mut self, top_p: f64) -> Result<Self, ValidationError> {
        if !(0.0..=1.0).contains(&top_p) {
            return Err(ValidationError::InvalidTopP(top_p));
        }
        self.request = self.request.with_top_p(top_p);
        Ok(self)
    }

    pub fn frequency_penalty(mut self, penalty: f64) -> Result<Self, ValidationError> {
        if !(-2.0..=2.0).contains(&penalty) {
            return Err(ValidationError::InvalidFrequencyPenalty(penalty));
        }
        self.request = self.request.with_frequency_penalty(penalty);
        Ok(self)
    }

    pub fn presence_penalty(mut self, penalty: f64) -> Result<Self, ValidationError> {
        if !(-2.0..=2.0).contains(&penalty) {
            return Err(ValidationError::InvalidPresencePenalty(penalty));
        }
        self.request = self.request.with_presence_penalty(penalty);
        Ok(self)
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.request = self.request.with_seed(seed);
        self
    }

    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.request = self.request.with_metadata(metadata);
        self
    }

    pub fn store(mut self, store: bool) -> Self {
        self.request = self.request.with_store(store);
        self
    }

    pub fn reasoning(mut self, effort: ReasoningEffort) -> Result<Self, ValidationError> {
        if !self.request.model.supports_reasoning() {
            return Err(ValidationError::ReasoningNotSupported(self.request.model));
        }
        self.request = self.request.with_reasoning(effort);
        Ok(self)
    }

    pub fn truncation(mut self, type_: TruncationType, last_turns: Option<u32>) -> Self {
        self.request = self.request.with_truncation(type_, last_turns);
        self
    }

    pub fn modalities(mut self, modalities: Vec<Modality>) -> Result<Self, ValidationError> {
        if modalities.contains(&Modality::Image) && !self.request.model.supports_vision() {
            return Err(ValidationError::VisionNotSupported(self.request.model));
        }
        self.request = self.request.with_modalities(modalities);
        Ok(self)
    }

    pub fn response_format_text(mut self) -> Self {
        self.request = self.request.with_response_format_text();
        self
    }

    pub fn response_format_json(mut self) -> Result<Self, ValidationError> {
        if !self.request.model.supports_json_mode() {
            return Err(ValidationError::JsonModeNotSupported(self.request.model));
        }
        self.request = self.request.with_response_format_json();
        Ok(self)
    }

    pub fn response_format_schema(mut self, schema: serde_json::Value) -> Result<Self, ValidationError> {
        if !self.request.model.supports_json_mode() {
            return Err(ValidationError::JsonModeNotSupported(self.request.model));
        }
        self.request = self.request.with_response_format_schema(schema);
        Ok(self)
    }

    pub fn parallel_tool_calls(mut self, parallel: bool) -> Result<Self, ValidationError> {
        if !self.request.model.supports_function_calling() {
            return Err(ValidationError::FunctionCallingNotSupported(self.request.model));
        }
        self.request = self.request.with_parallel_tool_calls(parallel);
        Ok(self)
    }

    pub fn tool_choice_auto(mut self) -> Result<Self, ValidationError> {
        if !self.request.model.supports_function_calling() {
            return Err(ValidationError::FunctionCallingNotSupported(self.request.model));
        }
        self.request = self.request.with_tool_choice_auto();
        Ok(self)
    }

    pub fn tool_choice_required(mut self) -> Result<Self, ValidationError> {
        if !self.request.model.supports_function_calling() {
            return Err(ValidationError::FunctionCallingNotSupported(self.request.model));
        }
        self.request = self.request.with_tool_choice_required();
        Ok(self)
    }

    pub fn tool_choice_none(mut self) -> Self {
        self.request = self.request.with_tool_choice_none();
        self
    }

    pub fn tool_choice_specific(mut self, name: impl Into<String>) -> Result<Self, ValidationError> {
        if !self.request.model.supports_function_calling() {
            return Err(ValidationError::FunctionCallingNotSupported(self.request.model));
        }
        self.request = self.request.with_tool_choice_specific(name);
        Ok(self)
    }

    pub fn tools(mut self, tools: Vec<Tool>) -> Result<Self, ValidationError> {
        if !self.request.model.supports_function_calling() {
            return Err(ValidationError::FunctionCallingNotSupported(self.request.model));
        }
        self.request = self.request.with_tools(tools);
        Ok(self)
    }

    pub fn build(self) -> Result<CreateResponseRequest, ValidationError> {
        self.request.validate_for_model()?;
        Ok(self.request)
    }
}

