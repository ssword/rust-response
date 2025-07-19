//! # OpenAI Responses API Rust SDK
//!
//! A comprehensive Rust SDK for the OpenAI Responses API.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use openai_responses::{OpenAIClient, OpenAIConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = OpenAIClient::from_env()?;
//!     let response = client
//!         .create_response_builder("gpt-4.1-nano", "Hello, world!")
//!         .temperature(0.7)
//!         .max_tokens(100)
//!         .send()
//!         .await?;
//!     
//!     if let Some(text) = response.get_text_output() {
//!         println!("Response: {}", text);
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod config;
pub mod error;
pub mod types;
pub mod endpoints;
pub mod json;

#[cfg(feature = "streaming")]
pub mod streaming;

#[cfg(feature = "backend-abstraction")]
pub mod backend;

pub use client::OpenAIClient;
pub use config::OpenAIConfig;
pub use error::{OpenAIError, Result};
pub use types::*;
pub use json::{AdaptiveJsonParser, JsonCapabilities};

#[cfg(feature = "streaming")]
pub use streaming::{ResponseStream, StreamingExt};

#[cfg(feature = "backend-abstraction")]
pub use backend::{HttpBackend, MockBackend, ReqwestBackend, MockResponseExt};

#[cfg(feature = "simd")]
pub use json::{SimdJsonParser, SimdJsonExt};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::client::OpenAIClient;
    pub use crate::config::OpenAIConfig;
    pub use crate::error::Result;
    pub use crate::types::*;
    pub use crate::endpoints::ResponseBuilder;
    
    #[cfg(feature = "streaming")]
    pub use crate::streaming::{ResponseStream, StreamingExt};
}