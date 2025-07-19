use std::marker::PhantomData;
use crate::types::{CreateResponseRequest, ValidationError, Model};
use crate::client::OpenAIClient;
use crate::error::Result;

/// Type-state markers for compile-time validation
pub struct Unvalidated;
pub struct Validated;

/// Type-safe request builder using phantom types
#[derive(Debug)]
pub struct TypedRequestBuilder<State = Unvalidated> {
    request: CreateResponseRequest,
    _state: PhantomData<State>,
}

impl TypedRequestBuilder<Unvalidated> {
    /// Create a new unvalidated request builder
    pub fn new(model: Model, input: impl Into<String>) -> Self {
        Self {
            request: CreateResponseRequest::new(model, input),
            _state: PhantomData,
        }
    }

    /// Add instructions to the request
    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.request = self.request.with_instructions(instructions);
        self
    }

    /// Set temperature with compile-time validation
    pub fn temperature(mut self, temperature: f64) -> Result<Self, ValidationError> {
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