use serde::{Deserialize, Serialize};
use std::fmt;

/// Reasoning effort level for models that support advanced reasoning.
///
/// This enum controls how much computational effort reasoning models
/// (like O1, O3Mini, O4Mini) put into solving complex problems.
/// Higher effort levels result in more thorough analysis but take longer.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{Model, ReasoningEffort};
///
/// let model = Model::O1;
/// if model.supports_reasoning() {
///     // Use high effort for complex problems
///     // request.with_reasoning(ReasoningEffort::High);
/// }
/// ```
///
/// # Effort Levels
///
/// * `Low` - Fast reasoning with basic analysis
/// * `Medium` - Balanced reasoning with moderate depth
/// * `High` - Thorough reasoning with deep analysis (slower but more accurate)
///
/// # Serialization
///
/// Serializes to lowercase strings: "low", "medium", "high"
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    /// Fast reasoning with basic analysis
    Low,
    /// Balanced reasoning with moderate depth
    Medium,
    /// Thorough reasoning with deep analysis
    High,
}

impl ReasoningEffort {
    /// Returns the string representation for API calls.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::ReasoningEffort;
    ///
    /// assert_eq!(ReasoningEffort::High.as_str(), "high");
    /// assert_eq!(ReasoningEffort::Low.as_str(), "low");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            ReasoningEffort::Low => "low",
            ReasoningEffort::Medium => "medium",
            ReasoningEffort::High => "high",
        }
    }
}

/// Display implementation for ReasoningEffort.
///
/// Formats the reasoning effort using its API string representation.
///
/// # Examples
///
/// ```rust
/// use openai_responses::ReasoningEffort;
///
/// let effort = ReasoningEffort::Medium;
/// assert_eq!(format!("{}", effort), "medium");
/// ```
impl fmt::Display for ReasoningEffort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Truncation strategy for conversation history management.
///
/// This enum controls how the API handles conversation context when it
/// exceeds the model's context window. It determines whether and how
/// to automatically truncate older messages to fit within limits.
///
/// # Examples
///
/// ```rust
/// use openai_responses::TruncationType;
///
/// // Let the API automatically manage context
/// // request.with_truncation(TruncationType::Auto, None);
///
/// // Disable truncation (may cause errors if context is too long)
/// // request.with_truncation(TruncationType::Disabled, None);
/// ```
///
/// # Variants
///
/// * `Auto` - Automatically truncate older messages when needed
/// * `Disabled` - Never truncate, may cause context length errors
///
/// # Serialization
///
/// Serializes to lowercase strings: "auto", "disabled"
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum TruncationType {
    /// Automatically truncate older messages when context limit is reached
    Auto,
    /// Never truncate messages, may cause context length errors
    Disabled,
}

impl TruncationType {
    /// Returns the string representation for API calls.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::TruncationType;
    ///
    /// assert_eq!(TruncationType::Auto.as_str(), "auto");
    /// assert_eq!(TruncationType::Disabled.as_str(), "disabled");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            TruncationType::Auto => "auto",
            TruncationType::Disabled => "disabled",
        }
    }
}

/// Display implementation for TruncationType.
///
/// Formats the truncation type using its API string representation.
///
/// # Examples
///
/// ```rust
/// use openai_responses::TruncationType;
///
/// let truncation = TruncationType::Auto;
/// assert_eq!(format!("{}", truncation), "auto");
/// ```
impl fmt::Display for TruncationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Response format type for controlling output structure.
///
/// This enum specifies how the model should format its response.
/// Different formats are useful for different use cases, from free-form
/// text to structured data extraction.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{Model, ResponseFormatType};
///
/// let model = Model::Gpt4_1Nano;
/// if model.supports_json_mode() {
///     // Request JSON object format
///     // request.with_response_format_json();
/// }
/// ```
///
/// # Variants
///
/// * `Text` - Free-form text response (default)
/// * `JsonObject` - Valid JSON object format
/// * `JsonSchema` - JSON conforming to a specific schema
///
/// # Model Support
///
/// Not all models support all response formats. Use [`Model::supports_json_mode()`]
/// to check if a model supports JSON formats.
///
/// # Serialization
///
/// Serializes to specific strings: "text", "json_object", "json_schema"
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ResponseFormatType {
    /// Free-form text response (default format)
    Text,
    /// Valid JSON object format
    #[serde(rename = "json_object")]
    JsonObject,
    /// JSON conforming to a specific schema
    #[serde(rename = "json_schema")]
    JsonSchema,
}

impl ResponseFormatType {
    /// Returns the string representation for API calls.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::ResponseFormatType;
    ///
    /// assert_eq!(ResponseFormatType::Text.as_str(), "text");
    /// assert_eq!(ResponseFormatType::JsonObject.as_str(), "json_object");
    /// assert_eq!(ResponseFormatType::JsonSchema.as_str(), "json_schema");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            ResponseFormatType::Text => "text",
            ResponseFormatType::JsonObject => "json_object",
            ResponseFormatType::JsonSchema => "json_schema",
        }
    }
}

/// Display implementation for ResponseFormatType.
///
/// Formats the response format type using its API string representation.
///
/// # Examples
///
/// ```rust
/// use openai_responses::ResponseFormatType;
///
/// let format = ResponseFormatType::JsonObject;
/// assert_eq!(format!("{}", format), "json_object");
/// ```
impl fmt::Display for ResponseFormatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Tool choice strategy for function calling.
///
/// This enum controls how the model should handle tool/function calling
/// when tools are provided. It determines whether the model must use tools,
/// can choose to use them, or should avoid them entirely.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{Model, ToolChoiceType};
///
/// let model = Model::Gpt4_1Nano;
/// if model.supports_function_calling() {
///     // Let the model decide when to use tools
///     // request.with_tool_choice_auto();
///
///     // Force the model to use a tool
///     // request.with_tool_choice_required();
/// }
/// ```
///
/// # Variants
///
/// * `Auto` - Model decides when to use tools (default)
/// * `Required` - Model must use at least one tool
/// * `None` - Model should not use any tools
///
/// # Model Support
///
/// Only models that support function calling can use tools.
/// Use [`Model::supports_function_calling()`] to check compatibility.
///
/// # Serialization
///
/// Serializes to lowercase strings: "auto", "required", "none"
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceType {
    /// Model decides when to use tools (default behavior)
    Auto,
    /// Model must use at least one tool in its response
    Required,
    /// Model should not use any tools
    None,
}

impl ToolChoiceType {
    /// Returns the string representation for API calls.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::ToolChoiceType;
    ///
    /// assert_eq!(ToolChoiceType::Auto.as_str(), "auto");
    /// assert_eq!(ToolChoiceType::Required.as_str(), "required");
    /// assert_eq!(ToolChoiceType::None.as_str(), "none");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            ToolChoiceType::Auto => "auto",
            ToolChoiceType::Required => "required",
            ToolChoiceType::None => "none",
        }
    }
}

/// Display implementation for ToolChoiceType.
///
/// Formats the tool choice type using its API string representation.
///
/// # Examples
///
/// ```rust
/// use openai_responses::ToolChoiceType;
///
/// let choice = ToolChoiceType::Required;
/// assert_eq!(format!("{}", choice), "required");
/// ```
impl fmt::Display for ToolChoiceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Input modality types for multimodal requests.
///
/// This enum specifies the types of input the model should expect and process.
/// Different models support different combinations of modalities.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{Model, Modality};
///
/// let model = Model::Gpt4_1;
/// if model.supports_vision() {
///     // Include both text and image inputs
///     // request.with_modalities(vec![Modality::Text, Modality::Image]);
/// }
/// ```
///
/// # Variants
///
/// * `Text` - Text-based input (supported by all models)
/// * `Image` - Image input for vision-capable models
/// * `Audio` - Audio input for audio-capable models
///
/// # Model Support
///
/// * Text: Supported by all models
/// * Image: Only vision-capable models (use [`Model::supports_vision()`])
/// * Audio: Only audio-capable models (check model documentation)
///
/// # Serialization
///
/// Serializes to lowercase strings: "text", "image", "audio"
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Modality {
    /// Text input (supported by all models)
    Text,
    /// Image input (requires vision-capable models)
    Image,
    /// Audio input (requires audio-capable models)
    Audio,
}

impl Modality {
    /// Returns the string representation for API calls.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::Modality;
    ///
    /// assert_eq!(Modality::Text.as_str(), "text");
    /// assert_eq!(Modality::Image.as_str(), "image");
    /// assert_eq!(Modality::Audio.as_str(), "audio");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Modality::Text => "text",
            Modality::Image => "image",
            Modality::Audio => "audio",
        }
    }
}

/// Display implementation for Modality.
///
/// Formats the modality using its API string representation.
///
/// # Examples
///
/// ```rust
/// use openai_responses::Modality;
///
/// let modality = Modality::Image;
/// assert_eq!(format!("{}", modality), "image");
/// ```
impl fmt::Display for Modality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test ReasoningEffort serialization and deserialization.
    #[test]
    fn test_reasoning_effort_serialization() {
        let effort = ReasoningEffort::High;
        let serialized = serde_json::to_string(&effort).unwrap();
        assert_eq!(serialized, "\"high\"");

        let deserialized: ReasoningEffort = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, ReasoningEffort::High);
    }

    /// Test TruncationType serialization and deserialization.
    #[test]
    fn test_truncation_type_serialization() {
        let truncation = TruncationType::Auto;
        let serialized = serde_json::to_string(&truncation).unwrap();
        assert_eq!(serialized, "\"auto\"");

        let deserialized: TruncationType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, TruncationType::Auto);
    }

    /// Test ResponseFormatType serialization with custom rename.
    #[test]
    fn test_response_format_type_serialization() {
        let format = ResponseFormatType::JsonObject;
        let serialized = serde_json::to_string(&format).unwrap();
        assert_eq!(serialized, "\"json_object\"");

        let deserialized: ResponseFormatType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, ResponseFormatType::JsonObject);
    }

    /// Test ToolChoiceType serialization and deserialization.
    #[test]
    fn test_tool_choice_type_serialization() {
        let choice = ToolChoiceType::Required;
        let serialized = serde_json::to_string(&choice).unwrap();
        assert_eq!(serialized, "\"required\"");

        let deserialized: ToolChoiceType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, ToolChoiceType::Required);
    }

    /// Test Modality serialization and deserialization.
    #[test]
    fn test_modality_serialization() {
        let modality = Modality::Image;
        let serialized = serde_json::to_string(&modality).unwrap();
        assert_eq!(serialized, "\"image\"");

        let deserialized: Modality = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, Modality::Image);
    }

    /// Test string representations.
    #[test]
    fn test_as_str_methods() {
        assert_eq!(ReasoningEffort::Medium.as_str(), "medium");
        assert_eq!(TruncationType::Disabled.as_str(), "disabled");
        assert_eq!(ResponseFormatType::JsonSchema.as_str(), "json_schema");
        assert_eq!(ToolChoiceType::Auto.as_str(), "auto");
        assert_eq!(Modality::Audio.as_str(), "audio");
    }

    /// Test Display implementations.
    #[test]
    fn test_display_implementations() {
        assert_eq!(format!("{}", ReasoningEffort::Low), "low");
        assert_eq!(format!("{}", TruncationType::Auto), "auto");
        assert_eq!(format!("{}", ResponseFormatType::Text), "text");
        assert_eq!(format!("{}", ToolChoiceType::None), "none");
        assert_eq!(format!("{}", Modality::Text), "text");
    }
}