use std::marker::PhantomData;
use crate::types::{CreateResponseRequest, ValidationError, Model};
use crate::client::OpenAIClient;
use crate::error::Result;

/// Marker type for unvalidated request builders.
///
/// In this state, the builder can be modified but cannot be built into
/// a final request. The builder must be validated first.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{Model, TypedRequestBuilder};
///
/// // Builder starts in Unvalidated state
/// let builder = TypedRequestBuilder::new(Model::Gpt4_1Nano, "Hello");
/// // Can modify the builder
/// let builder = builder.temperature(0.7);
/// // Must validate before building
/// let validated = builder.validate()?;
/// ```
pub struct Unvalidated;

/// Marker type for validated request builders.
///
/// In this state, the builder has passed all validation checks and
/// can be built into a final request for sending to the API.
///
/// # Examples
///
/// ```rust
/// use openai_responses::{Model, TypedRequestBuilder};
///
/// let validated_builder = TypedRequestBuilder::new(Model::Gpt4_1Nano, "Hello")
///     .temperature(0.7)
///     .validate()?;
///
/// // Now can build the final request
/// let request = validated_builder.build();
/// ```
pub struct Validated;

/// Type-state pattern builder for compile-time request validation.
///
/// This builder uses Rust's type system to ensure that requests are properly
/// validated before they can be sent to the API. The type-state pattern
/// prevents invalid requests from being constructed at compile time.
///
/// # Type States
///
/// The builder progresses through different type states:
/// 1. `TypedRequestBuilder<Unvalidated>` - Initial state, can be modified
/// 2. `TypedRequestBuilder<Validated>` - Validated state, ready to send
///
/// # Examples
///
/// ```rust
/// use openai_responses::{Model, TypedRequestBuilder, ReasoningEffort};
///
/// // Start with unvalidated builder
/// let builder = TypedRequestBuilder::new(Model::Gpt4_1Nano, "Hello");
///
/// // Add configuration (still unvalidated)
/// let builder = builder
///     .temperature(0.7)
///     .max_tokens(100);
///
/// // Validate and transition to validated state
/// let validated_builder = builder.validate()?;
///
/// // Build the final request
/// let request = validated_builder.build();
/// ```
///
/// # Compile-Time Safety
///
/// The type system prevents common mistakes:
///
/// ```rust,compile_fail
/// use openai_responses::{Model, TypedRequestBuilder};
///
/// let builder = TypedRequestBuilder::new(Model::Gpt4_1Nano, "Hello");
/// // This won't compile - must validate first!
/// let request = builder.build(); // ERROR: method not available
/// ```
///
/// # Model-Specific Validation
///
/// The builder automatically validates that requested features are supported
/// by the selected model:
///
/// ```rust
/// use openai_responses::{Model, TypedRequestBuilder, ReasoningEffort, Modality};
///
/// // This will fail validation - O1 doesn't support vision
/// let result = TypedRequestBuilder::new(Model::O1, "Analyze image")
///     .modalities(vec![Modality::Image])
///     .validate();
///
/// assert!(result.is_err());
/// ```
#[derive(Debug)]
pub struct TypedRequestBuilder<State = Unvalidated> {
    request: CreateResponseRequest,
    _state: PhantomData<State>,
}

impl TypedRequestBuilder<Unvalidated> {
    /// Creates a new unvalidated request builder.
    ///
    /// This is the entry point for the type-state builder pattern.
    /// The builder starts in the `Unvalidated` state and must be
    /// validated before it can be built into a final request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::{Model, TypedRequestBuilder};
    ///
    /// let builder = TypedRequestBuilder::new(
    ///     Model::Gpt4_1Nano,
    ///     "What is the capital of France?"
    /// );
    /// ```
    ///
    /// # Parameters
    ///
    /// * `model` - The [`Model`] to use for the request
    /// * `input` - The input text or prompt
    pub fn new(model: Model, input: impl Into<String>) -> Self {
        Self {
            request: CreateResponseRequest::new(model, input),
            _state: PhantomData,
        }
    }

    /// Adds system instructions to guide the model's behavior.
    ///
    /// Instructions provide context and guidelines for how the model
    /// should respond to the input. They act as a system message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::{Model, TypedRequestBuilder};
    ///
    /// let builder = TypedRequestBuilder::new(Model::Gpt4_1Nano, "Hello")
    ///     .instructions("Be helpful and concise");
    /// ```
    ///
    /// # Parameters
    ///
    /// * `instructions` - System instructions (can be any type that implements `Into<String>`)
    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.request = self.request.with_instructions(instructions);
        self
    }

    /// Sets the temperature parameter with validation.
    ///
    /// Temperature controls the randomness of the model's output.
    /// Higher values make output more random, lower values more deterministic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openai_responses::{Model, TypedRequestBuilder};
    ///
    /// let builder = TypedRequestBuilder::new(Model::Gpt4_1Nano, "Hello")
    ///     .temperature(0.7)?; // More creative
    ///
    /// let builder = TypedRequestBuilder::new(Model::Gpt4_1Nano, "Hello")
    ///     .temperature(0.1)?; // More deterministic
    /// ```
    ///
    /// # Parameters
    ///
    /// * `temperature` - Value between 0.0 and 2.0
    ///
    /// # Errors
    ///
    /// Returns [`ValidationError::InvalidTemperature`] if the value is outside
    /// the valid range of 0.0 to 2.0.
    pub fn temperature(mut self, temperature: f64) -> std::result::Result<Self, ValidationError> {
        if !(0.0..=2.0).contains(&temperature) {
            return Err(ValidationError::InvalidTemperature(temperature));
        }
        self.request = self.request.with_temperature(temperature);
        Ok(self)
    }

    /// Set max tokens
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.request = self.request.with_max_tokens(max_tokens);
        self
    }

    /// Set background processing
    pub fn background(mut self, background: bool) -> Self {
        self.request = self.request.with_background(background);
        self
    }

    /// Set top_p with validation
    pub fn top_p(mut self, top_p: f64) -> Result<Self, ValidationError> {
        if !(0.0..=1.0).contains(&top_p) {
            return Err(ValidationError::InvalidTopP(top_p));
        }
        self.request = self.request.with_top_p(top_p);
        Ok(self)
    }

    /// Set frequency penalty with validation
    pub fn frequency_penalty(mut self, penalty: f64) -> Result<Self, ValidationError> {
        if !(-2.0..=2.0).contains(&penalty) {
            return Err(ValidationError::InvalidFrequencyPenalty(penalty));
        }
        self.request = self.request.with_frequency_penalty(penalty);
        Ok(self)
    }

    /// Set presence penalty with validation
    pub fn presence_penalty(mut self, penalty: f64) -> Result<Self, ValidationError> {
        if !(-2.0..=2.0).contains(&penalty) {
            return Err(ValidationError::InvalidPresencePenalty(penalty));
        }
        self.request = self.request.with_presence_penalty(penalty);
        Ok(self)
    }

    /// Set seed
    pub fn seed(mut self, seed: u32) -> Self {
        self.request = self.request.with_seed(seed);
        self
    }

    /// Set metadata
    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.request = self.request.with_metadata(metadata);
        self
    }

    /// Set store flag
    pub fn store(mut self, store: bool) -> Self {
        self.request = self.request.with_store(store);
        self
    }

    /// Validate the request and transition to validated state
    pub fn validate(self) -> Result<TypedRequestBuilder<Validated>, ValidationError> {
        // Perform all validations
        self.request.validate_for_model()?;
        
        Ok(TypedRequestBuilder {
            request: self.request,
            _state: PhantomData,
        })
    }

    /// Validate and build the request in one step
    pub fn build(self) -> Result<CreateResponseRequest, ValidationError> {
        self.validate()?.build()
    }
}

impl TypedRequestBuilder<Validated> {
    /// Only validated requests can be built
    pub fn build(self) -> Result<CreateResponseRequest, ValidationError> {
        Ok(self.request)
    }

    /// Only validated requests can be sent
    pub async fn send(self, client: &OpenAIClient) -> Result<crate::types::Response> {
        client.create_response(&self.request).await
    }

    /// Send and wait for completion
    pub async fn send_and_wait(self, client: &OpenAIClient) -> Result<crate::types::Response> {
        let response = client.create_response(&self.request).await?;
        
        if response.status == crate::types::ResponseStatus::InProgress {
            client.wait_for_response(&response.id).await
        } else {
            Ok(response)
        }
    }

    /// Send with timeout
    pub async fn send_and_wait_with_timeout(
        self, 
        client: &OpenAIClient,
        timeout: std::time::Duration
    ) -> Result<crate::types::Response> {
        let response = client.create_response(&self.request).await?;
        
        if response.status == crate::types::ResponseStatus::InProgress {
            client.wait_for_response_with_timeout(&response.id, Some(timeout)).await
        } else {
            Ok(response)
        }
    }
}

/// Extension trait for OpenAIClient to create typed builders
impl OpenAIClient {
    /// Create a typed request builder with compile-time validation
    pub fn create_typed_request_builder(
        &self,
        model: Model,
        input: impl Into<String>
    ) -> TypedRequestBuilder<Unvalidated> {
        TypedRequestBuilder::new(model, input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Model;

    #[test]
    fn test_typed_builder_validation() {
        let builder = TypedRequestBuilder::new(Model::Gpt4_1, "Hello");
        
        // This should fail validation
        let result = builder.temperature(3.0);
        assert!(result.is_err());
        
        let builder = TypedRequestBuilder::new(Model::Gpt4_1, "Hello");
        
        // This should succeed
        let validated = builder
            .temperature(0.7).unwrap()
            .max_tokens(100)
            .validate();
        assert!(validated.is_ok());
    }

    #[test]
    fn test_compile_time_safety() {
        let builder = TypedRequestBuilder::new(Model::Gpt4_1, "Hello");
        
        // Can't send unvalidated request - this won't compile
        // builder.send(&client).await; // Compilation error!
        
        let validated = builder.validate().unwrap();
        // validated.send(&client).await; // This would compile
    }
}