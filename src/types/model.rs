use serde::{Deserialize, Serialize};
use std::fmt;

/// OpenAI model variants supported by the Responses API.
///
/// This enum provides type-safe model selection with compile-time validation
/// of model capabilities. Each model has different features and limitations
/// that are enforced through the type system.
///
/// # Examples
///
/// ```rust
/// use openai_responses::Model;
///
/// // Create a request with a specific model
/// let model = Model::Gpt4_1Nano;
///
/// // Check model capabilities at compile time
/// if model.supports_reasoning() {
///     println!("This model supports reasoning");
/// }
///
/// // Get the API string representation
/// assert_eq!(model.as_str(), "gpt-4.1-nano");
/// ```
///
/// # Model Capabilities
///
/// Different models support different features:
/// - **Reasoning**: O1, O3Mini, O4Mini models support advanced reasoning
/// - **Vision**: GPT-4.1 family models support image processing
/// - **JSON Mode**: Most models support structured JSON output
/// - **Function Calling**: Most models support tool/function calling
///
/// Use the capability methods to check what features are available:
/// - [`supports_reasoning()`](Model::supports_reasoning)
/// - [`supports_vision()`](Model::supports_vision)
/// - [`supports_json_mode()`](Model::supports_json_mode)
/// - [`supports_function_calling()`](Model::supports_function_calling)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Model {
    /// GPT-4.1 model with vision and function calling support
    Gpt4_1,
    /// GPT-4.1 Mini - smaller, faster version with vision support
    Gpt4_1Mini,
    /// GPT-4.1 Nano - smallest, fastest version with vision support
    Gpt4_1Nano,
    /// GPT-4o model with vision and function calling support
    Gpt4O,
    /// GPT-4o Mini - smaller version with vision support
    Gpt4OMini,
    /// O1 reasoning model - advanced reasoning capabilities, no vision
    O1,
    /// O3 Mini reasoning model - reasoning with JSON and function support
    O3Mini,
    /// O4 Mini reasoning model - reasoning with JSON and function support
    O4Mini,
}

impl Model {
    /// Returns the string representation of the model for API calls.
    ///
    /// This method provides the exact string that should be sent to the OpenAI API
    /// to identify the model. The returned string is guaranteed to be valid for
    /// the current API version.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::Model;
    ///
    /// assert_eq!(Model::Gpt4_1Nano.as_str(), "gpt-4.1-nano");
    /// assert_eq!(Model::O1.as_str(), "o1");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Model::Gpt4_1 => "gpt-4.1",
            Model::Gpt4_1Mini => "gpt-4.1-mini",
            Model::Gpt4_1Nano => "gpt-4.1-nano",
            Model::Gpt4O => "gpt-4o",
            Model::Gpt4OMini => "gpt-4o-mini",
            Model::O1 => "o1",
            Model::O3Mini => "o3-mini",
            Model::O4Mini => "o4-mini",
        }
    }

    /// Checks if the model supports advanced reasoning capabilities.
    ///
    /// Reasoning models (O1, O3Mini, O4Mini) can perform complex multi-step
    /// reasoning tasks and provide detailed thought processes. These models
    /// are optimized for problems requiring logical deduction, mathematical
    /// reasoning, and strategic thinking.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::Model;
    ///
    /// assert!(Model::O1.supports_reasoning());
    /// assert!(Model::O3Mini.supports_reasoning());
    /// assert!(!Model::Gpt4_1Nano.supports_reasoning());
    /// ```
    ///
    /// # Usage with Reasoning Configuration
    ///
    /// ```rust
    /// use openai_responses::{Model, ReasoningEffort};
    ///
    /// let model = Model::O1;
    /// if model.supports_reasoning() {
    ///     // Can use reasoning configuration
    ///     // request.with_reasoning(ReasoningEffort::High);
    /// }
    /// ```
    pub const fn supports_reasoning(&self) -> bool {
        matches!(self, Model::O1 | Model::O3Mini | Model::O4Mini)
    }

    /// Checks if the model supports vision capabilities (image processing).
    ///
    /// Vision-capable models can analyze images, understand visual content,
    /// and answer questions about images. They support multimodal inputs
    /// combining text and images.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::Model;
    ///
    /// assert!(Model::Gpt4_1.supports_vision());
    /// assert!(Model::Gpt4O.supports_vision());
    /// assert!(!Model::O1.supports_vision());
    /// ```
    ///
    /// # Usage with Image Modalities
    ///
    /// ```rust
    /// use openai_responses::{Model, Modality};
    ///
    /// let model = Model::Gpt4_1Nano;
    /// if model.supports_vision() {
    ///     // Can include image modality
    ///     // request.with_modalities(vec![Modality::Text, Modality::Image]);
    /// }
    /// ```
    pub const fn supports_vision(&self) -> bool {
        matches!(
            self,
            Model::Gpt4_1 | Model::Gpt4_1Mini | Model::Gpt4_1Nano | Model::Gpt4O | Model::Gpt4OMini
        )
    }

    /// Checks if the model supports JSON mode for structured output.
    ///
    /// JSON mode allows the model to return responses in valid JSON format,
    /// which is useful for structured data extraction, API responses, and
    /// integration with other systems.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::Model;
    ///
    /// assert!(Model::Gpt4_1.supports_json_mode());
    /// assert!(Model::O3Mini.supports_json_mode());
    /// assert!(!Model::O1.supports_json_mode());
    /// ```
    ///
    /// # Usage with Response Format
    ///
    /// ```rust
    /// use openai_responses::Model;
    ///
    /// let model = Model::Gpt4_1Nano;
    /// if model.supports_json_mode() {
    ///     // Can set JSON response format
    ///     // request.with_response_format_json();
    /// }
    /// ```
    pub const fn supports_json_mode(&self) -> bool {
        matches!(
            self,
            Model::Gpt4_1
                | Model::Gpt4_1Mini
                | Model::Gpt4_1Nano
                | Model::Gpt4O
                | Model::Gpt4OMini
                | Model::O3Mini
                | Model::O4Mini
        )
    }

    /// Checks if the model supports function calling (tool usage).
    ///
    /// Function calling allows the model to call external tools and functions
    /// to perform specific tasks like web searches, calculations, or API calls.
    /// This enables more interactive and capable AI applications.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::Model;
    ///
    /// assert!(Model::Gpt4_1.supports_function_calling());
    /// assert!(Model::O3Mini.supports_function_calling());
    /// assert!(!Model::O1.supports_function_calling());
    /// ```
    ///
    /// # Usage with Tools
    ///
    /// ```rust
    /// use openai_responses::Model;
    ///
    /// let model = Model::Gpt4_1Nano;
    /// if model.supports_function_calling() {
    ///     // Can add tools to the request
    ///     // request.with_tools(tools);
    /// }
    /// ```
    pub const fn supports_function_calling(&self) -> bool {
        matches!(
            self,
            Model::Gpt4_1
                | Model::Gpt4_1Mini
                | Model::Gpt4_1Nano
                | Model::Gpt4O
                | Model::Gpt4OMini
                | Model::O3Mini
                | Model::O4Mini
        )
    }

    /// Returns the maximum context window size for the model in tokens.
    ///
    /// The context window determines how much text (input + output) the model
    /// can process in a single request. Larger context windows allow for
    /// longer conversations and documents.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::Model;
    ///
    /// assert_eq!(Model::Gpt4_1.max_context_window(), 1_000_000);
    /// assert_eq!(Model::Gpt4O.max_context_window(), 128_000);
    /// assert_eq!(Model::O1.max_context_window(), 200_000);
    /// ```
    ///
    /// # Usage for Token Management
    ///
    /// ```rust
    /// use openai_responses::Model;
    ///
    /// let model = Model::Gpt4_1Nano;
    /// let max_tokens = model.max_context_window();
    ///
    /// // Ensure request fits within context window
    /// if input_tokens + max_output_tokens <= max_tokens {
    ///     // Safe to proceed with request
    /// }
    /// ```
    pub const fn max_context_window(&self) -> usize {
        match self {
            Model::Gpt4_1 => 1_000_000,
            Model::Gpt4_1Mini => 1_000_000,
            Model::Gpt4_1Nano => 1_000_000,
            Model::Gpt4O => 128_000,
            Model::Gpt4OMini => 128_000,
            Model::O1 => 200_000,
            Model::O3Mini => 200_000,
            Model::O4Mini => 200_000,
        }
    }

    /// Returns a slice containing all available models.
    ///
    /// This method provides access to all model variants supported by the SDK.
    /// Useful for iteration, validation, or building dynamic model selection UIs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::Model;
    ///
    /// // Iterate over all models
    /// for model in Model::all() {
    ///     println!("Model: {} (supports reasoning: {})",
    ///              model.as_str(),
    ///              model.supports_reasoning());
    /// }
    ///
    /// // Check if a model is valid
    /// let model_name = "gpt-4.1-nano";
    /// let is_valid = Model::all().iter()
    ///     .any(|m| m.as_str() == model_name);
    /// ```
    pub const fn all() -> &'static [Model] {
        &[
            Model::Gpt4_1,
            Model::Gpt4_1Mini,
            Model::Gpt4_1Nano,
            Model::Gpt4O,
            Model::Gpt4OMini,
            Model::O1,
            Model::O3Mini,
            Model::O4Mini,
        ]
    }

    /// Returns models filtered by a specific capability.
    ///
    /// This method allows you to find all models that support a particular
    /// feature, which is useful for dynamic model selection based on
    /// application requirements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::{Model, ModelCapability};
    ///
    /// // Get all reasoning-capable models
    /// let reasoning_models = Model::with_capability(ModelCapability::Reasoning);
    /// assert!(reasoning_models.contains(&Model::O1));
    ///
    /// // Get all vision-capable models
    /// let vision_models = Model::with_capability(ModelCapability::Vision);
    /// assert!(vision_models.contains(&Model::Gpt4_1));
    /// ```
    ///
    /// # Parameters
    ///
    /// * `capability` - The [`ModelCapability`] to filter by
    ///
    /// # Returns
    ///
    /// A `Vec<Model>` containing all models that support the specified capability.
    pub fn with_capability(capability: ModelCapability) -> Vec<Model> {
        Model::all()
            .iter()
            .copied()
            .filter(|model| match capability {
                ModelCapability::Reasoning => model.supports_reasoning(),
                ModelCapability::Vision => model.supports_vision(),
                ModelCapability::JsonMode => model.supports_json_mode(),
                ModelCapability::FunctionCalling => model.supports_function_calling(),
            })
            .collect()
    }
}

/// Display implementation for Model.
///
/// Formats the model using its API string representation.
/// This is equivalent to calling [`as_str()`](Model::as_str).
///
/// # Examples
///
/// ```rust
/// use openai_responses::Model;
///
/// let model = Model::Gpt4_1Nano;
/// assert_eq!(format!("{}", model), "gpt-4.1-nano");
/// ```
impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Serde serialization implementation for Model.
///
/// Serializes the model as its API string representation, ensuring
/// compatibility with the OpenAI API format.
///
/// # Examples
///
/// ```rust
/// use openai_responses::Model;
///
/// let model = Model::O1;
/// let json = serde_json::to_string(&model).unwrap();
/// assert_eq!(json, "\"o1\"");
/// ```
impl Serialize for Model {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Serde deserialization implementation for Model.
///
/// Deserializes a model from its API string representation.
/// Returns an error if the string doesn't match any known model.
///
/// # Examples
///
/// ```rust
/// use openai_responses::Model;
///
/// let json = "\"gpt-4.1-nano\"";
/// let model: Model = serde_json::from_str(json).unwrap();
/// assert_eq!(model, Model::Gpt4_1Nano);
/// ```
impl<'de> Deserialize<'de> for Model {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

/// FromStr implementation for Model.
///
/// Parses a model from its API string representation.
/// This is the inverse of [`as_str()`](Model::as_str).
///
/// # Examples
///
/// ```rust
/// use openai_responses::Model;
/// use std::str::FromStr;
///
/// let model = Model::from_str("gpt-4.1-nano").unwrap();
/// assert_eq!(model, Model::Gpt4_1Nano);
///
/// // Also works with parse()
/// let model: Model = "o1".parse().unwrap();
/// assert_eq!(model, Model::O1);
///
/// // Invalid models return an error
/// assert!(Model::from_str("invalid-model").is_err());
/// ```
///
/// # Errors
///
/// Returns a `String` error if the input doesn't match any known model name.
impl std::str::FromStr for Model {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gpt-4.1" => Ok(Model::Gpt4_1),
            "gpt-4.1-mini" => Ok(Model::Gpt4_1Mini),
            "gpt-4.1-nano" => Ok(Model::Gpt4_1Nano),
            "gpt-4o" => Ok(Model::Gpt4O),
            "gpt-4o-mini" => Ok(Model::Gpt4OMini),
            "o1" => Ok(Model::O1),
            "o3-mini" => Ok(Model::O3Mini),
            "o4-mini" => Ok(Model::O4Mini),
            _ => Err(format!("Unknown model: {}", s)),
        }
    }
}

/// Model capability categories for filtering and validation.
///
/// This enum represents different capabilities that models may or may not support.
/// It's used with [`Model::with_capability()`] to find models that support
/// specific features.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{Model, ModelCapability};
///
/// // Find all models that support reasoning
/// let reasoning_models = Model::with_capability(ModelCapability::Reasoning);
///
/// // Find all models that support vision
/// let vision_models = Model::with_capability(ModelCapability::Vision);
/// ```
///
/// # Variants
///
/// * `Reasoning` - Advanced reasoning and multi-step problem solving
/// * `Vision` - Image processing and multimodal inputs
/// * `JsonMode` - Structured JSON output format
/// * `FunctionCalling` - Tool usage and external function calls
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelCapability {
    /// Advanced reasoning capabilities for complex problem solving
    Reasoning,
    /// Vision processing for image analysis and multimodal inputs
    Vision,
    /// JSON mode for structured output format
    JsonMode,
    /// Function calling for tool usage and external API integration
    FunctionCalling,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that models serialize to their correct API string representation
    /// and can be deserialized back to the same model.
    #[test]
    fn test_model_serialization() {
        let model = Model::Gpt4_1;
        let serialized = serde_json::to_string(&model).unwrap();
        assert_eq!(serialized, "\"gpt-4.1\"");

        let deserialized: Model = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, Model::Gpt4_1);
    }

    /// Test that model capabilities are correctly identified.
    #[test]
    fn test_model_capabilities() {
        // Test reasoning capabilities
        assert!(Model::O1.supports_reasoning());
        assert!(Model::O3Mini.supports_reasoning());
        assert!(!Model::Gpt4O.supports_reasoning());
        assert!(!Model::Gpt4_1Nano.supports_reasoning());

        // Test vision capabilities
        assert!(Model::Gpt4O.supports_vision());
        assert!(Model::Gpt4_1.supports_vision());
        assert!(!Model::O1.supports_vision());
        assert!(!Model::O3Mini.supports_vision());

        // Test JSON mode capabilities
        assert!(Model::Gpt4_1.supports_json_mode());
        assert!(Model::O3Mini.supports_json_mode());
        assert!(!Model::O1.supports_json_mode());

        // Test function calling capabilities
        assert!(Model::Gpt4_1.supports_function_calling());
        assert!(Model::O3Mini.supports_function_calling());
        assert!(!Model::O1.supports_function_calling());
    }

    /// Test parsing models from strings.
    #[test]
    fn test_model_from_str() {
        assert_eq!("gpt-4.1".parse::<Model>().unwrap(), Model::Gpt4_1);
        assert_eq!("gpt-4.1-nano".parse::<Model>().unwrap(), Model::Gpt4_1Nano);
        assert_eq!("o1".parse::<Model>().unwrap(), Model::O1);
        assert!("invalid-model".parse::<Model>().is_err());
    }

    /// Test model capability filtering.
    #[test]
    fn test_model_capability_filtering() {
        let reasoning_models = Model::with_capability(ModelCapability::Reasoning);
        assert!(reasoning_models.contains(&Model::O1));
        assert!(reasoning_models.contains(&Model::O3Mini));
        assert!(!reasoning_models.contains(&Model::Gpt4_1));

        let vision_models = Model::with_capability(ModelCapability::Vision);
        assert!(vision_models.contains(&Model::Gpt4_1));
        assert!(vision_models.contains(&Model::Gpt4O));
        assert!(!vision_models.contains(&Model::O1));
    }

    /// Test context window sizes.
    #[test]
    fn test_context_windows() {
        assert_eq!(Model::Gpt4_1.max_context_window(), 1_000_000);
        assert_eq!(Model::Gpt4O.max_context_window(), 128_000);
        assert_eq!(Model::O1.max_context_window(), 200_000);
    }
}