use serde::{Deserialize, Serialize};
use super::model::Model;
use super::enums::{ReasoningEffort, TruncationType, ResponseFormatType, ToolChoiceType, Modality};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponseRequest {
    pub model: Model,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<TruncationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modality>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReasoningConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<ReasoningEffort>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TruncationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<TruncationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_turns: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponseFormat {
    Text { type_: ResponseFormatType },
    JsonObject { type_: ResponseFormatType },
    JsonSchema {
        type_: ResponseFormatType,
        json_schema: serde_json::Value,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ToolChoice {
    Auto,
    Required,
    None,
    Specific { type_: ToolChoiceType, name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<ToolFunction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolFunction {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

impl CreateResponseRequest {
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
            type_: "json_schema".to_string(),
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

#[derive(Debug, Clone, thiserror::Error)]
pub enum ValidationError {
    #[error("Model {0} does not support reasoning capabilities")]
    ReasoningNotSupported(Model),
    
    #[error("Model {0} does not support vision capabilities")]
    VisionNotSupported(Model),
    
    #[error("Model {0} does not support JSON mode")]
    JsonModeNotSupported(Model),
    
    #[error("Model {0} does not support function calling")]
    FunctionCallingNotSupported(Model),
    
    #[error("Temperature {0} must be between 0.0 and 2.0")]
    InvalidTemperature(f64),
    
    #[error("Top-p {0} must be between 0.0 and 1.0")]
    InvalidTopP(f64),
    
    #[error("Frequency penalty {0} must be between -2.0 and 2.0")]
    InvalidFrequencyPenalty(f64),
    
    #[error("Presence penalty {0} must be between -2.0 and 2.0")]
    InvalidPresencePenalty(f64),
}

/// Builder for CreateResponseRequest with model-specific defaults and validation
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

    /// Validates the request configuration against the selected model capabilities
    pub fn validate_for_model(&self,
    ) -> Result<(), ValidationError> {
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

    /// Returns a builder with model-specific defaults
    pub fn builder(model: Model, input: impl Into<String>) -> CreateResponseRequestBuilder {
        CreateResponseRequestBuilder::new(model, input)
    }
}