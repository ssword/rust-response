# OpenAI Responses API Rust SDK

A comprehensive, async-first Rust SDK for the OpenAI Responses API.

## Features

- **Async/await** support with tokio
- **Type-safe Model enum** with compile-time validation and capability checking
- **Comprehensive type safety** with serde
- **Retry logic** with exponential backoff
- **Configurable** via environment variables or builder pattern
- **Full API coverage** including create, retrieve, delete, cancel, and list responses
- **Advanced error handling** with detailed error types and optional enhanced errors
- **Fluent API** with builder pattern and typed builders
- **Concurrent support** for multiple requests with Arc-based client sharing
- **Streaming support** (optional feature) for real-time response processing
- **Background processing** with polling and timeout support
- **Rust-specific optimizations** including SIMD JSON parsing and backend abstraction

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
openai-responses = "0.1.0"
```

## Quick Start

### Basic Usage

```rust
use openai_responses::{OpenAIClient, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::from_env()?;

    let response = client
        .create_response_builder(Model::Gpt4_1Nano, "Hello, world!")
        .send()
        .await?;

    println!("Response: {}", response.get_text_output().unwrap_or_default());
    Ok(())
}
```

### Configuration

```rust
use openai_responses::{OpenAIClient, OpenAIConfig};
use std::time::Duration;

let config = OpenAIConfig::new("your-api-key")
    .with_timeout(Duration::from_secs(30))
    .with_max_retries(3)
    .with_base_url("https://api.openai.com/v1");

let client = OpenAIClient::new(config)?;
```

### Environment Variables

```bash
export OPENAI_API_KEY="your-api-key"
export OPENAI_ORGANIZATION="your-org-id"
export OPENAI_PROJECT="your-project-id"
```

## API Reference

### Client Creation

```rust
// From environment variables
let client = OpenAIClient::from_env()?;

// With custom prefix
let client = OpenAIClient::from_env_with_prefix("CUSTOM_OPENAI")?;

// With configuration
let config = OpenAIConfig::new("api-key");
let client = OpenAIClient::new(config)?;
```

### Creating Responses

```rust
use openai_responses::{Model, ReasoningEffort, TruncationType, Modality};

// Simple creation
let response = client.create_simple_response(Model::Gpt4_1Nano, "Hello").await?;

// With builder pattern
let response = client
    .create_response_builder(Model::Gpt4_1Nano, "Write a story")
    .temperature(0.7)
    .max_tokens(200)
    .instructions("Be creative and engaging")
    .send()
    .await?;

// With background processing
let response = client
    .create_response_builder(Model::Gpt4_1Nano, "Long task")
    .background(true)
    .send()
    .await?;

// Advanced features with reasoning models
let response = client
    .create_response_builder(Model::O1, "Solve this complex problem")
    .reasoning(ReasoningEffort::High)
    .max_tokens(1000)
    .send_and_wait()
    .await?;

// Wait for completion with timeout
let completed = client.wait_for_response_with_timeout(&response.id,
    Some(std::time::Duration::from_secs(60))).await?;
```

### Managing Responses

```rust
// Retrieve a response
let response = client.get_response("resp_123").await?;

// List responses with pagination
let response_list = client.list_responses(Some(10), None, None).await?;
for response in response_list.data {
    println!("Response {}: {:?}", response.id, response.status);
}

// Delete a response
let result = client.delete_response("resp_123").await?;
assert!(result.deleted);

// Cancel an in-progress response
let response = client.cancel_response("resp_123").await?;
```

## Examples

Run the examples:

```bash
# Basic usage
cargo run --example response_simple

# Basic response patterns
cargo run --example response_basics

# Async/concurrent usage
cargo run --example response_async_batch

# Background processing and streaming
cargo run --example response_streaming

# Rust-specific features demonstration
cargo run --example response_rust_features

# Text generation and prompting
cargo run --example response_text_generation

# Conversation management
cargo run --example response_conversation

# Function calling and tools
cargo run --example response_function_calls

# Structured JSON output
cargo run --example response_json_output

# Web search integration
cargo run --example response_web_search

# File analysis and search
cargo run --example response_file_analysis

# Advanced reasoning
cargo run --example response_reasoning

# Multimodal capabilities
cargo run --example response_multimodal

# Model validation
cargo run --example response_model_validation

# Lazy parsing optimization
cargo run --example response_lazy_parsing
```

## Error Handling

The SDK provides comprehensive error handling:

```rust
use openai_responses::{OpenAIClient, OpenAIError, Model};

match client.create_simple_response(Model::Gpt4_1Nano, "Hello").await {
    Ok(response) => println!("Success: {}", response.id),
    Err(OpenAIError::Authentication(msg)) => eprintln!("Auth error: {}", msg),
    Err(OpenAIError::RateLimit(msg)) => eprintln!("Rate limit: {}", msg),
    Err(OpenAIError::InvalidRequest(msg)) => eprintln!("Invalid request: {}", msg),
    Err(OpenAIError::NotFound(msg)) => eprintln!("Not found: {}", msg),
    Err(OpenAIError::ServerError(msg)) => eprintln!("Server error: {}", msg),
    Err(OpenAIError::Timeout(msg)) => eprintln!("Timeout: {}", msg),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Testing

```bash
cargo test
```

## Cargo Features

- `rustls` (default): Use rustls for TLS
- `native-tls`: Use native TLS
- `enhanced-errors`: Enhanced error context with anyhow
- `simd`: SIMD-accelerated JSON parsing
- `streaming`: Real-time response streaming support
- `backend-abstraction`: Abstract backend trait for testing

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please read the contributing guidelines and submit pull requests.