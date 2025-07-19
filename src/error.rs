use thiserror::Error;
use reqwest::StatusCode;

#[derive(Debug, Error)]
pub enum OpenAIError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),
    
    #[error("API error: {message}")]
    Api {
        message: String,
        code: Option<String>,
        status: Option<StatusCode>,
    },
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Server error: {0}")]
    ServerError(String),
    
    #[error("Timeout error: {0}")]
    Timeout(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Streaming error: {0}")]
    Streaming(String),
    
    #[error("Response cancelled: {0}")]
    Cancelled(String),
}

impl OpenAIError {
    pub fn is_retryable(&self) -> bool {
        match self {
            OpenAIError::Http(_) => true,
            OpenAIError::ServerError(_) => true,
            OpenAIError::RateLimit(_) => true,
            OpenAIError::Timeout(_) => true,
            OpenAIError::Api { status, .. } => {
                status.map_or(false, |s| {
                    s.is_server_error() || s == StatusCode::TOO_MANY_REQUESTS
                })
            }
            _ => false,
        }
    }

    pub fn from_api_response(status: StatusCode, error_body: &str) -> Self {
        if let Ok(api_error) = serde_json::from_str::<crate::types::ErrorResponse>(error_body) {
            match status {
                StatusCode::UNAUTHORIZED => {
                    OpenAIError::Authentication(api_error.error.message)
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    OpenAIError::RateLimit(api_error.error.message)
                }
                StatusCode::BAD_REQUEST => {
                    OpenAIError::InvalidRequest(api_error.error.message)
                }
                StatusCode::NOT_FOUND => {
                    OpenAIError::NotFound(api_error.error.message)
                }
                s if s.is_server_error() => {
                    OpenAIError::ServerError(api_error.error.message)
                }
                _ => {
                    OpenAIError::Api {
                        message: api_error.error.message,
                        code: api_error.error.code,
                        status: Some(status),
                    }
                }
            }
        } else {
            OpenAIError::Api {
                message: error_body.to_string(),
                code: None,
                status: Some(status),
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, OpenAIError>;