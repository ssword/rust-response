use thiserror::Error;
use reqwest::StatusCode;

#[cfg(feature = "enhanced-errors")]
use anyhow::{Context as AnyhowContext, Result as AnyhowResult};

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
    
    #[cfg(feature = "enhanced-errors")]
    #[error("Context error: {0}")]
    WithContext(#[from] anyhow::Error),
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
            #[cfg(feature = "enhanced-errors")]
            OpenAIError::WithContext(e) => {
                // Check if the root cause is retryable
                if let Some(openai_err) = e.downcast_ref::<OpenAIError>() {
                    openai_err.is_retryable()
                } else {
                    false
                }
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

    /// Get the HTTP status code if available
    pub fn status_code(&self) -> Option<StatusCode> {
        match self {
            OpenAIError::Api { status, .. } => *status,
            OpenAIError::Http(e) => e.status(),
            _ => None,
        }
    }

    /// Get the error code if available
    pub fn error_code(&self) -> Option<&str> {
        match self {
            OpenAIError::Api { code, .. } => code.as_deref(),
            _ => None,
        }
    }

    /// Check if this is a client error (4xx)
    pub fn is_client_error(&self) -> bool {
        self.status_code().map_or(false, |s| s.is_client_error())
    }

    /// Check if this is a server error (5xx)
    pub fn is_server_error(&self) -> bool {
        self.status_code().map_or(false, |s| s.is_server_error())
    }
}

pub type Result<T> = std::result::Result<T, OpenAIError>;

#[cfg(feature = "enhanced-errors")]
pub type ContextResult<T> = AnyhowResult<T>;

/// Extension trait for adding context to errors
#[cfg(feature = "enhanced-errors")]
pub trait ErrorContextExt<T> {
    /// Add context to an error
    fn with_context<F>(self, f: F) -> ContextResult<T>
    where
        F: FnOnce() -> String;
    
    /// Add static context to an error
    fn context(self, msg: &'static str) -> ContextResult<T>;
}

#[cfg(feature = "enhanced-errors")]
impl<T> ErrorContextExt<T> for Result<T> {
    fn with_context<F>(self, f: F) -> ContextResult<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| anyhow::Error::new(e).context(f()))
    }
    
    fn context(self, msg: &'static str) -> ContextResult<T> {
        self.map_err(|e| anyhow::Error::new(e).context(msg))
    }
}

#[cfg(feature = "enhanced-errors")]
impl<T, E> ErrorContextExt<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn with_context<F>(self, f: F) -> ContextResult<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| anyhow::Error::new(e).context(f()))
    }
    
    fn context(self, msg: &'static str) -> ContextResult<T> {
        self.map_err(|e| anyhow::Error::new(e).context(msg))
    }
}

/// Circuit breaker pattern for error recovery
#[cfg(feature = "enhanced-errors")]
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    failure_threshold: usize,
    success_threshold: usize,
    timeout: std::time::Duration,
    state: CircuitBreakerState,
    failure_count: usize,
    success_count: usize,
    last_failure_time: Option<std::time::Instant>,
}

#[cfg(feature = "enhanced-errors")]
#[derive(Debug, Clone, PartialEq)]
enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}

#[cfg(feature = "enhanced-errors")]
impl CircuitBreaker {
    pub fn new(failure_threshold: usize, success_threshold: usize, timeout: std::time::Duration) -> Self {
        Self {
            failure_threshold,
            success_threshold,
            timeout,
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
        }
    }

    pub fn can_execute(&mut self) -> bool {
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {
                if let Some(last_failure) = self.last_failure_time {
                    if last_failure.elapsed() >= self.timeout {
                        self.state = CircuitBreakerState::HalfOpen;
                        self.success_count = 0;
                        true
                    } else {
                        false
                    }
                } else {
                    true
                }
            }
            CircuitBreakerState::HalfOpen => true,
        }
    }

    pub fn record_success(&mut self) {
        match self.state {
            CircuitBreakerState::Closed => {
                self.failure_count = 0;
            }
            CircuitBreakerState::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= self.success_threshold {
                    self.state = CircuitBreakerState::Closed;
                    self.failure_count = 0;
                }
            }
            CircuitBreakerState::Open => {}
        }
    }

    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(std::time::Instant::now());
        
        match self.state {
            CircuitBreakerState::Closed => {
                if self.failure_count >= self.failure_threshold {
                    self.state = CircuitBreakerState::Open;
                }
            }
            CircuitBreakerState::HalfOpen => {
                self.state = CircuitBreakerState::Open;
            }
            CircuitBreakerState::Open => {}
        }
    }

    pub fn is_open(&self) -> bool {
        self.state == CircuitBreakerState::Open
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_retryable() {
        assert!(OpenAIError::RateLimit("test".to_string()).is_retryable());
        assert!(OpenAIError::ServerError("test".to_string()).is_retryable());
        assert!(!OpenAIError::Authentication("test".to_string()).is_retryable());
        assert!(!OpenAIError::InvalidRequest("test".to_string()).is_retryable());
    }

    #[test]
    fn test_error_status_code() {
        let error = OpenAIError::Api {
            message: "test".to_string(),
            code: None,
            status: Some(StatusCode::BAD_REQUEST),
        };
        
        assert_eq!(error.status_code(), Some(StatusCode::BAD_REQUEST));
        assert!(error.is_client_error());
        assert!(!error.is_server_error());
    }

    #[cfg(feature = "enhanced-errors")]
    #[test]
    fn test_circuit_breaker() {
        let mut breaker = CircuitBreaker::new(3, 2, std::time::Duration::from_millis(100));
        
        // Initially closed
        assert!(breaker.can_execute());
        assert!(!breaker.is_open());
        
        // Record failures
        breaker.record_failure();
        breaker.record_failure();
        assert!(breaker.can_execute());
        assert!(!breaker.is_open());
        
        // Third failure should open the circuit
        breaker.record_failure();
        assert!(!breaker.can_execute());
        assert!(breaker.is_open());
    }

    #[cfg(feature = "enhanced-errors")]
    #[test]
    fn test_error_context() {
        use crate::error::ErrorContextExt;
        
        let result: Result<()> = Err(OpenAIError::InvalidRequest("test".to_string()));
        let with_context = result.context("Failed to create request");
        
        assert!(with_context.is_err());
        let error_string = format!("{:#}", with_context.unwrap_err());
        assert!(error_string.contains("Failed to create request"));
        assert!(error_string.contains("Invalid request: test"));
    }
}