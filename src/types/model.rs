use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Model {
    Gpt4_1,
    Gpt4_1Mini,
    Gpt4_1Nano,
    Gpt4O,
    Gpt4OMini,
    O1,
    O3Mini,
    O4Mini,
}

impl Model {
    /// Returns the string representation of the model for API calls
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

    /// Returns all models that support reasoning capabilities
    pub const fn supports_reasoning(&self) -> bool {
        matches!(self, Model::O1 | Model::O3Mini | Model::O4Mini)
    }

    /// Returns all models that support vision capabilities
    pub const fn supports_vision(&self) -> bool {
        matches!(
            self,
            Model::Gpt4_1 | Model::Gpt4_1Mini | Model::Gpt4_1Nano | Model::Gpt4O | Model::Gpt4OMini
        )
    }

    /// Returns all models that support JSON mode
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

    /// Returns all models that support function calling
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

    /// Returns maximum context window for the model
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

    /// Returns all available models
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

    /// Returns models filtered by capability
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

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for Model {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Model {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Model::from_str(&s).map_err(serde::de::Error::custom)
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelCapability {
    Reasoning,
    Vision,
    JsonMode,
    FunctionCalling,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_serialization() {
        let model = Model::Gpt4_1;
        let serialized = serde_json::to_string(&model).unwrap();
        assert_eq!(serialized, "\"gpt-4.1\"");

        let deserialized: Model = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, Model::Gpt4_1);
    }

    #[test]
    fn test_model_capabilities() {
        assert!(Model::O1.supports_reasoning());
        assert!(!Model::Gpt4O.supports_reasoning());
        
        assert!(Model::Gpt4O.supports_vision());
        assert!(!Model::O1.supports_vision());
    }

    #[test]
    fn test_model_from_str() {
        assert_eq!("gpt-4.1".parse::<Model>().unwrap(), Model::Gpt4_1);
        assert!("invalid-model".parse::<Model>().is_err());
    }
}