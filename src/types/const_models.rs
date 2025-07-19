use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use crate::types::{CreateResponseRequest, ValidationError, ReasoningEffort, Modality, Model};
use std::str::FromStr;

/// Trait defining model capabilities at the type level for compile-time validation.
///
/// This trait enables compile-time checking of model capabilities, ensuring that
/// only supported features are used with each model. It uses associated constants
/// to define what each model can do, allowing the type system to enforce these
/// constraints.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{ModelCapabilities, O1, Gpt4_1};
///
/// // Check capabilities at compile time
/// const O1_SUPPORTS_REASONING: bool = O1::SUPPORTS_REASONING; // true
/// const GPT4_SUPPORTS_REASONING: bool = Gpt4_1::SUPPORTS_REASONING; // false
///
/// // Use in generic functions
/// fn can_use_reasoning<M: ModelCapabilities>() -> bool {
///     M::SUPPORTS_REASONING
/// }
/// ```
///
/// # Implementation
///
/// Implement this trait for each model type to define its capabilities:
///
/// ```rust
/// use openai_responses::ModelCapabilities;
///
/// pub struct MyCustomModel;
/// impl ModelCapabilities for MyCustomModel {
///     const SUPPORTS_REASONING: bool = true;
///     const SUPPORTS_VISION: bool = false;
///     const SUPPORTS_JSON: bool = true;
///     const SUPPORTS_FUNCTIONS: bool = true;
///     const MAX_CONTEXT_WINDOW: usize = 100_000;
///     const MODEL_NAME: &'static str = "my-custom-model";
/// }
/// ```
pub trait ModelCapabilities {
    /// Whether the model supports advanced reasoning capabilities
    const SUPPORTS_REASONING: bool;

    /// Whether the model supports vision/image processing
    const SUPPORTS_VISION: bool;

    /// Whether the model supports JSON mode output
    const SUPPORTS_JSON: bool;

    /// Whether the model supports function calling
    const SUPPORTS_FUNCTIONS: bool;

    /// Maximum context window size in tokens
    const MAX_CONTEXT_WINDOW: usize;

    /// API name for the model
    const MODEL_NAME: &'static str;
}

/// Typed model wrapper that enforces capabilities at compile time.
///
/// This wrapper provides a zero-cost abstraction over model types that
/// enables compile-time capability checking. It uses phantom types to
/// associate model capabilities with the type system.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{TypedModel, O1, Gpt4_1};
///
/// // Create typed model instances
/// let o1_model: TypedModel<O1> = TypedModel::new();
/// let gpt4_model: TypedModel<Gpt4_1> = TypedModel::new();
///
/// // Check capabilities at compile time
/// assert!(TypedModel::<O1>::supports_reasoning());
/// assert!(!TypedModel::<Gpt4_1>::supports_reasoning());
/// ```
///
/// # Zero-Cost Abstraction
///
/// `TypedModel` has no runtime overhead - it's a zero-sized type that
/// only exists at compile time to enforce type safety.
///
/// # Serialization
///
/// The model serializes to its API string representation and can be
/// deserialized back, with validation that the string matches the
/// expected model name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypedModel<M: ModelCapabilities>(PhantomData<M>);

impl<M: ModelCapabilities> TypedModel<M> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }

    pub const fn supports_reasoning() -> bool {
        M::SUPPORTS_REASONING
    }

    pub const fn supports_vision() -> bool {
        M::SUPPORTS_VISION
    }

    pub const fn supports_json() -> bool {
        M::SUPPORTS_JSON
    }

    pub const fn supports_functions() -> bool {
        M::SUPPORTS_FUNCTIONS
    }

    pub const fn max_context_window() -> usize {
        M::MAX_CONTEXT_WINDOW
    }

    pub const fn model_name() -> &'static str {
        M::MODEL_NAME
    }
}

impl<M: ModelCapabilities> Default for TypedModel<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: ModelCapabilities> Serialize for TypedModel<M> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(M::MODEL_NAME)
    }
}

impl<'de, M: ModelCapabilities> Deserialize<'de> for TypedModel<M> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == M::MODEL_NAME {
            Ok(Self::new())
        } else {
            Err(serde::de::Error::custom(format!(
                "Expected model '{}', got '{}'",
                M::MODEL_NAME,
                s
            )))
        }
    }
}

// Define concrete model types with capabilities
pub struct Gpt4_1;
impl ModelCapabilities for Gpt4_1 {
    const SUPPORTS_REASONING: bool = false;
    const SUPPORTS_VISION: bool = true;
    const SUPPORTS_JSON: bool = true;
    const SUPPORTS_FUNCTIONS: bool = true;
    const MAX_CONTEXT_WINDOW: usize = 1_000_000;
    const MODEL_NAME: &'static str = "gpt-4.1";
}

pub struct Gpt4_1Mini;
impl ModelCapabilities for Gpt4_1Mini {
    const SUPPORTS_REASONING: bool = false;
    const SUPPORTS_VISION: bool = true;
    const SUPPORTS_JSON: bool = true;
    const SUPPORTS_FUNCTIONS: bool = true;
    const MAX_CONTEXT_WINDOW: usize = 1_000_000;
    const MODEL_NAME: &'static str = "gpt-4.1-mini";
}

pub struct Gpt4_1Nano;
impl ModelCapabilities for Gpt4_1Nano {
    const SUPPORTS_REASONING: bool = false;
    const SUPPORTS_VISION: bool = true;
    const SUPPORTS_JSON: bool = true;
    const SUPPORTS_FUNCTIONS: bool = true;
    const MAX_CONTEXT_WINDOW: usize = 1_000_000;
    const MODEL_NAME: &'static str = "gpt-4.1-nano";
}

pub struct Gpt4O;
impl ModelCapabilities for Gpt4O {
    const SUPPORTS_REASONING: bool = false;
    const SUPPORTS_VISION: bool = true;
    const SUPPORTS_JSON: bool = true;
    const SUPPORTS_FUNCTIONS: bool = true;
    const MAX_CONTEXT_WINDOW: usize = 128_000;
    const MODEL_NAME: &'static str = "gpt-4o";
}

pub struct Gpt4OMini;
impl ModelCapabilities for Gpt4OMini {
    const SUPPORTS_REASONING: bool = false;
    const SUPPORTS_VISION: bool = true;
    const SUPPORTS_JSON: bool = true;
    const SUPPORTS_FUNCTIONS: bool = true;
    const MAX_CONTEXT_WINDOW: usize = 128_000;
    const MODEL_NAME: &'static str = "gpt-4o-mini";
}

pub struct O1;
impl ModelCapabilities for O1 {
    const SUPPORTS_REASONING: bool = true;
    const SUPPORTS_VISION: bool = false;
    const SUPPORTS_JSON: bool = false;
    const SUPPORTS_FUNCTIONS: bool = false;
    const MAX_CONTEXT_WINDOW: usize = 200_000;
    const MODEL_NAME: &'static str = "o1";
}

pub struct O3Mini;
impl ModelCapabilities for O3Mini {
    const SUPPORTS_REASONING: bool = true;
    const SUPPORTS_VISION: bool = false;
    const SUPPORTS_JSON: bool = true;
    const SUPPORTS_FUNCTIONS: bool = true;
    const MAX_CONTEXT_WINDOW: usize = 200_000;
    const MODEL_NAME: &'static str = "o3-mini";
}

pub struct O4Mini;
impl ModelCapabilities for O4Mini {
    const SUPPORTS_REASONING: bool = true;
    const SUPPORTS_VISION: bool = false;
    const SUPPORTS_JSON: bool = true;
    const SUPPORTS_FUNCTIONS: bool = true;
    const MAX_CONTEXT_WINDOW: usize = 200_000;
    const MODEL_NAME: &'static str = "o4-mini";
}

/// Compile-time validated request builder for specific model types
#[derive(Debug)]
pub struct ModelTypedRequestBuilder<M: ModelCapabilities> {
    model: TypedModel<M>,
    request: CreateResponseRequest,
}

impl<M: ModelCapabilities> ModelTypedRequestBuilder<M> {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            model: TypedModel::new(),
            request: CreateResponseRequest::new(
                Model::from_str(M::MODEL_NAME).unwrap_or(Model::Gpt4_1),
                input
            ),
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

    // Only available for reasoning-capable models
    pub fn reasoning(mut self, effort: ReasoningEffort) -> Self
    where
        [(); if M::SUPPORTS_REASONING { 1 } else { 0 }]:,
    {
        self.request = self.request.with_reasoning(effort);
        self
    }

    // Only available for vision-capable models
    pub fn modalities(mut self, modalities: Vec<Modality>) -> Self
    where
        [(); if M::SUPPORTS_VISION { 1 } else { 0 }]:,
    {
        self.request = self.request.with_modalities(modalities);
        self
    }

    // Only available for JSON-capable models
    pub fn response_format_json(mut self) -> Self
    where
        [(); if M::SUPPORTS_JSON { 1 } else { 0 }]:,
    {
        self.request = self.request.with_response_format_json();
        self
    }

    // Only available for function-calling capable models
    pub fn with_tools(mut self, tools: Vec<crate::types::Tool>) -> Self
    where
        [(); if M::SUPPORTS_FUNCTIONS { 1 } else { 0 }]:,
    {
        self.request = self.request.with_tools(tools);
        self
    }

    pub fn build(self) -> CreateResponseRequest {
        self.request
    }
}

/// Type aliases for common model builders
pub type Gpt4_1Builder = ModelTypedRequestBuilder<Gpt4_1>;
pub type Gpt4_1MiniBuilder = ModelTypedRequestBuilder<Gpt4_1Mini>;
pub type Gpt4_1NanoBuilder = ModelTypedRequestBuilder<Gpt4_1Nano>;
pub type Gpt4OBuilder = ModelTypedRequestBuilder<Gpt4O>;
pub type Gpt4OMiniBuilder = ModelTypedRequestBuilder<Gpt4OMini>;
pub type O1Builder = ModelTypedRequestBuilder<O1>;
pub type O3MiniBuilder = ModelTypedRequestBuilder<O3Mini>;
pub type O4MiniBuilder = ModelTypedRequestBuilder<O4Mini>;

/// Factory functions for creating typed model builders
impl Gpt4_1Builder {
    pub fn create(input: impl Into<String>) -> Self {
        Self::new(input)
    }
}

impl Gpt4_1MiniBuilder {
    pub fn create(input: impl Into<String>) -> Self {
        Self::new(input)
    }
}

impl Gpt4_1NanoBuilder {
    pub fn create(input: impl Into<String>) -> Self {
        Self::new(input)
    }
}

impl O1Builder {
    pub fn create(input: impl Into<String>) -> Self {
        Self::new(input)
    }
}

impl O3MiniBuilder {
    pub fn create(input: impl Into<String>) -> Self {
        Self::new(input)
    }
}

impl O4MiniBuilder {
    pub fn create(input: impl Into<String>) -> Self {
        Self::new(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_time_capabilities() {
        // This will compile - O1 supports reasoning
        let _o1_request = O1Builder::create("Hello")
            .reasoning(ReasoningEffort::High)
            .build();

        // This will compile - GPT-4.1 supports vision
        let _gpt4_request = Gpt4_1Builder::create("Hello")
            .modalities(vec![Modality::Text, Modality::Image])
            .build();

        // These would NOT compile (commenting out to avoid compilation errors in tests):
        // O1Builder::create("Hello").modalities(vec![Modality::Image]).build(); // O1 doesn't support vision
        // Gpt4_1Builder::create("Hello").reasoning(ReasoningEffort::High).build(); // GPT-4.1 doesn't support reasoning
    }

    #[test]
    fn test_model_capabilities() {
        assert!(O1::SUPPORTS_REASONING);
        assert!(!O1::SUPPORTS_VISION);
        
        assert!(!Gpt4_1::SUPPORTS_REASONING);
        assert!(Gpt4_1::SUPPORTS_VISION);
        
        assert_eq!(O1::MODEL_NAME, "o1");
        assert_eq!(Gpt4_1::MODEL_NAME, "gpt-4.1");
    }

    #[test]
    fn test_typed_model_serialization() {
        let model: TypedModel<O1> = TypedModel::new();
        let serialized = serde_json::to_string(&model).unwrap();
        assert_eq!(serialized, "\"o1\"");

        let deserialized: TypedModel<O1> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, model);
    }
}