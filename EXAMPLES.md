# OpenAI Responses API Rust SDK Examples

This directory contains comprehensive Rust examples that mirror the Python examples from the OpenAI Responses API GitHub repository.

## 📋 Quick Start

Set your OpenAI API key:
```bash
export OPENAI_API_KEY="your-api-key-here"
```

## 🎯 Example Overview

| Example | Description | Key Features |
|---------|-------------|--------------|
| **response_simple.rs** | Basic response creation and simple usage | Simple text generation, basic error handling |
| **response_basics.rs** | Basic response patterns and API usage | Fundamental API calls, parameter tuning |
| **response_text_generation.rs** | Advanced text prompting techniques | Temperature control, max tokens, system prompts |
| **response_conversation.rs** | Multi-turn conversations with context | State management, role-based conversations |
| **response_function_calls.rs** | Tool use and function calling | Weather, calculator, database simulation |
| **response_json_output.rs** | JSON schema validation and structured responses | Person profiles, recipes, sentiment analysis |
| **response_web_search.rs** | Web search integration | Current events, market research, news summarization |
| **response_file_analysis.rs** | File content analysis and search | Code review, documentation extraction, security scanning |
| **response_reasoning.rs** | Complex reasoning and multi-step problems | Mathematical solving, ethical analysis, strategic planning |
| **response_multimodal.rs** | Multimodal capabilities | Text and image processing, vision tasks |
| **response_model_validation.rs** | Model capability validation | Compile-time checks, runtime validation |
| **response_lazy_parsing.rs** | Zero-copy response parsing | Performance optimization, memory efficiency |
| **response_async_batch.rs** | Async/concurrent processing | Batch requests, concurrent execution |
| **response_streaming.rs** | Real-time response streaming | Server-sent events, incremental processing |
| **response_rust_features.rs** | Rust-specific features demonstration | Type safety, memory optimization, async patterns |

## 🚀 Running Examples

### Individual Examples
```bash
# Basic examples
cargo run --example response_simple
cargo run --example response_basics
cargo run --example response_text_generation

# Advanced examples
cargo run --example response_conversation
cargo run --example response_function_calls
cargo run --example response_json_output

# Specialized examples
cargo run --example response_web_search
cargo run --example response_file_analysis
cargo run --example response_reasoning
cargo run --example response_multimodal

# Performance and validation examples
cargo run --example response_model_validation
cargo run --example response_lazy_parsing
cargo run --example response_async_batch
cargo run --example response_streaming
cargo run --example response_rust_features
```

### All Examples
```bash
# Run all examples sequentially
for example in response_simple response_basics response_text_generation response_conversation response_function_calls response_json_output response_web_search response_file_analysis response_reasoning response_multimodal response_model_validation response_lazy_parsing response_async_batch response_streaming response_rust_features; do
    echo "=== Running $example ==="
    cargo run --example "$example"
done
```

## 🔧 Environment Configuration

### Basic Setup
```bash
export OPENAI_API_KEY="sk-..."              # Required
export OPENAI_ORGANIZATION="your-org"       # Optional
export OPENAI_PROJECT="your-project"        # Optional
export OPENAI_BASE_URL="https://api.openai.com/v1"  # Optional
```

### Alternative Prefix-Based Configuration
```bash
export CUSTOM_OPENAI_API_KEY="sk-..."
export CUSTOM_OPENAI_ORGANIZATION="your-org"
```

Then use:
```rust
let client = OpenAIClient::from_env_with_prefix("CUSTOM_OPENAI")?;
```

## 📊 Example Categories

### 🔰 **Beginner Examples**
- **response_simple.rs**: Perfect for first-time users
- **response_basics.rs**: Learn basic API patterns
- **response_text_generation.rs**: Learn parameter tuning

### 🗣️ **Conversation Examples**
- **response_conversation.rs**: Stateful conversations, context management

### 🛠️ **Advanced Features**
- **response_function_calls.rs**: Tool integration and external APIs
- **response_json_output.rs**: JSON validation and data extraction
- **response_web_search.rs**: Real-time information retrieval
- **response_file_analysis.rs**: Document analysis and code review

### 🧠 **Reasoning Examples**
- **response_reasoning.rs**: Complex problem solving and strategic analysis

### 🦀 **Rust-Specific Examples**
- **response_rust_features.rs**: Demonstration of Rust-specific features and optimizations
- **response_async_batch.rs**: Async/concurrent usage patterns
- **response_streaming.rs**: Background processing and streaming responses
- **response_model_validation.rs**: Model capability validation
- **response_lazy_parsing.rs**: Zero-copy parsing optimization

## 📝 Example Usage Patterns

### Basic Usage

```rust
use openai_responses::{OpenAIClient, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::from_env()?;

    let response = client
        .create_simple_response(Model::Gpt4_1Nano, "Hello, world!")
        .await?;

    println!("Response: {}", response.get_text_output().unwrap_or_default());
    Ok(())
}
```

### Advanced Builder Pattern

```rust
use openai_responses::{Model, ReasoningEffort, Modality};

let response = client
    .create_response_builder(Model::Gpt4_1Nano, "Your prompt here")
    .temperature(0.7)
    .max_tokens(200)
    .instructions("System instructions here")
    .send()
    .await?;

// For reasoning models
let reasoning_response = client
    .create_response_builder(Model::O1, "Complex problem to solve")
    .reasoning(ReasoningEffort::High)
    .send_and_wait()
    .await?;
```

### Error Handling

```rust
use openai_responses::{OpenAIError, Model};

match client.create_simple_response(Model::Gpt4_1Nano, "Hello").await {
    Ok(response) => println!("Success: {}", response.id),
    Err(OpenAIError::Authentication(msg)) => eprintln!("Auth error: {}", msg),
    Err(OpenAIError::RateLimit(msg)) => eprintln!("Rate limit: {}", msg),
    Err(e) => eprintln!("Error: {}", e),
}
```

## 🔍 Key Features Demonstrated

### Core Functionality
- ✅ Response creation and retrieval
- ✅ Response deletion and cleanup
- ✅ Background processing with polling
- ✅ Comprehensive error handling

### Advanced Features
- ✅ Function calling with custom tools
- ✅ JSON schema validation
- ✅ Web search integration
- ✅ File content analysis
- ✅ Multi-turn conversations
- ✅ Structured data extraction

### Rust-Specific Features
- ✅ Async/await throughout with `tokio`
- ✅ Type-safe Model enum with compile-time validation
- ✅ Proper error handling with `thiserror` and `ValidationError`
- ✅ Type-safe JSON serialization with `serde`
- ✅ Configurable retry logic with `backon` exponential backoff
- ✅ Environment variable configuration with `from_env()`
- ✅ Builder pattern API with fluent method chaining
- ✅ Zero-copy parsing with `LazyResponse` and `RawValue`
- ✅ Optional SIMD JSON parsing for performance
- ✅ Streaming support with `futures::Stream`

## 🧪 Testing

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test integration_tests

# All tests
cargo test
```

## 🎓 Learning Path

1. **Start with 01_introduction.rs** - Get familiar with basic API usage
2. **Progress to 02_text_prompting.rs** - Learn parameter tuning
3. **Try 03_conversation_state.rs** - Understand context management
4. **Explore 04_function_calling.rs** - Master tool integration
5. **Use 05_structured_output.rs** - Handle structured data
6. **Apply 06_web_search.rs** - Integrate real-time information
7. **Analyze with 07_file_search.rs** - Process documents and code
8. **Solve with 08_reasoning.rs** - Tackle complex problems

## 📚 Documentation Links

- [OpenAI Responses API Reference](https://platform.openai.com/docs/api-reference/responses)
- [Rust SDK API Docs](https://docs.rs/openai-responses)
- [GitHub Repository](https://github.com/Jaimboh/OpenAI-Responses-API)