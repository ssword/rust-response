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
| **01_introduction.rs** | Basic response creation and simple usage | Simple text generation, basic error handling |
| **02_text_prompting.rs** | Advanced text prompting techniques | Temperature control, max tokens, system prompts |
| **03_conversation_state.rs** | Multi-turn conversations with context | State management, role-based conversations |
| **04_function_calling.rs** | Tool use and function calling | Weather, calculator, database simulation |
| **05_structured_output.rs** | JSON schema validation and structured responses | Person profiles, recipes, sentiment analysis |
| **06_web_search.rs** | Web search integration | Current events, market research, news summarization |
| **07_file_search.rs** | File content analysis and search | Code review, documentation extraction, security scanning |
| **08_reasoning.rs** | Complex reasoning and multi-step problems | Mathematical solving, ethical analysis, strategic planning |

## 🚀 Running Examples

### Individual Examples
```bash
# Basic examples
cargo run --example 01_introduction
cargo run --example 02_text_prompting

# Advanced examples
cargo run --example 03_conversation_state
cargo run --example 04_function_calling
cargo run --example 05_structured_output

# Specialized examples
cargo run --example 06_web_search
cargo run --example 07_file_search
cargo run --example 08_reasoning

# Rust-specific examples
cargo run --example basic_usage
cargo run --example async_example
cargo run --example streaming
cargo run --example rust_features_demo
```

### All Examples
```bash
# Run all examples sequentially
for i in {01..08}; do
    echo "=== Running example $i ==="
    cargo run --example "${i}_$(echo $i | sed 's/01/introduction/;s/02/text_prompting/;s/03/conversation_state/;s/04/function_calling/;s/05/structured_output/;s/06/web_search/;s/07/file_search/;s/08/reasoning/')"
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
- **01_introduction.rs**: Perfect for first-time users
- **02_text_prompting.rs**: Learn parameter tuning

### 🗣️ **Conversation Examples**
- **03_conversation_state.rs**: Stateful conversations, context management

### 🛠️ **Advanced Features**
- **04_function_calling.rs**: Tool integration and external APIs
- **05_structured_output.rs**: JSON validation and data extraction
- **06_web_search.rs**: Real-time information retrieval
- **07_file_search.rs**: Document analysis and code review

### 🧠 **Reasoning Examples**
- **08_reasoning.rs**: Complex problem solving and strategic analysis

### 🦀 **Rust-Specific Examples**
- **basic_usage.rs**: Simple usage patterns and basic API calls
- **async_example.rs**: Async/concurrent usage patterns
- **streaming.rs**: Background processing and streaming responses
- **rust_features_demo.rs**: Demonstration of Rust-specific features and optimizations

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