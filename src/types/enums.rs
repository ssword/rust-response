use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    Low,
    Medium,
    High,
}

impl ReasoningEffort {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReasoningEffort::Low => "low",
            ReasoningEffort::Medium => "medium",
            ReasoningEffort::High => "high",
        }
    }
}

impl fmt::Display for ReasoningEffort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum TruncationType {
    Auto,
    Disabled,
}

impl TruncationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TruncationType::Auto => "auto",
            TruncationType::Disabled => "disabled",
        }
    }
}

impl fmt::Display for TruncationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ResponseFormatType {
    Text,
    JsonObject,
    JsonSchema,
}

impl ResponseFormatType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResponseFormatType::Text => "text",
            ResponseFormatType::JsonObject => "json_object",
            ResponseFormatType::JsonSchema => "json_schema",
        }
    }
}

impl fmt::Display for ResponseFormatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceType {
    Auto,
    Required,
    None,
}

impl ToolChoiceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ToolChoiceType::Auto => "auto",
            ToolChoiceType::Required => "required",
            ToolChoiceType::None => "none",
        }
    }
}

impl fmt::Display for ToolChoiceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Modality {
    Text,
    #[serde(rename = "text")]
    Image,
    #[serde(rename = "image")]
    Audio,
}

impl Modality {
    pub fn as_str(&self) -> &'static str {
        match self {
            Modality::Text => "text",
            Modality::Image => "image",
            Modality::Audio => "audio",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reasoning_effort_serialization() {
        let effort = ReasoningEffort::High;
        let serialized = serde_json::to_string(&effort).unwrap();
        assert_eq!(serialized, "\"high\"");

        let deserialized: ReasoningEffort = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, ReasoningEffort::High);
    }

    #[test]
    fn test_truncation_type_serialization() {
        let truncation = TruncationType::Auto;
        let serialized = serde_json::to_string(&truncation).unwrap();
        assert_eq!(serialized, "\"auto\"");

        let deserialized: TruncationType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, TruncationType::Auto);
    }
}