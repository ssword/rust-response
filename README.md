# OpenAI Responses API Rust SDK

A comprehensive, async-first Rust SDK for the OpenAI Responses API.

## Features

- **Async/await** support with tokio
- **Comprehensive type safety** with serde
- **Retry logic** with exponential backoff
- **Configurable** via environment variables or builder pattern
- **Full API coverage** including create, retrieve, delete, and cancel responses
- **Error handling** with detailed error types
- **Fluent API** with builder pattern
- **Concurrent support** for multiple requests

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
openai-responses = "0.1.0"
```

## Quick Start

### Basic Usage

```rust
use openai_responses::OpenAIClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::from_env()?;
    
    let response = client
        .create_response_builder("gpt-4.1-nano", "Hello, world!")
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
// Simple creation
let response = client.create_simple_response("gpt-4.1-nano", "Hello").await?;

// With builder pattern
let response = client
    .create_response_builder("gpt-4.1-nano", "Write a story")
    .temperature(0.7)
    .max_tokens(200)
    .send()
    .await?;

// With background processing
let response = client
    .create_response_builder("gpt-4.1-nano", "Long task")
    .background(true)
    .send()
    .await?;

// Wait for completion
let completed = client.wait_for_response(&response.id).await?;
```

### Managing Responses

```rust
// Retrieve a response
let response = client.get_response("resp_123").await?;

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
cargo run --example basic_usage

# Async/concurrent usage
cargo run --example async_example

# Background processing
cargo run --example streaming
```

## Error Handling

The SDK provides comprehensive error handling:

```rust
use openai_responses::{OpenAIClient, OpenAIError};

match client.create_simple_response("gpt-4.1-nano", "Hello").await {
    Ok(response) => println!("Success: {}", response.id),
    Err(OpenAIError::Authentication(msg)) => eprintln!("Auth error: {}", msg),
    Err(OpenAIError::RateLimit(msg)) => eprintln!("Rate limit: {}", msg),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Testing

```bash
cargo test
```

## Features

- `rustls` (default): Use rustls for TLS
- `native-tls`: Use native TLS

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please read the contributing guidelines and submit pull requests.