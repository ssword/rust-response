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

## 📝 Example Usage Patterns

### Basic Usage
```rust
use openai_responses::OpenAIClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::from_env()?;
    
    let response = client
        .create_simple_response("gpt-4.1-nano", "Hello, world!")
        .await?;
    
    println!("Response: {}", response.get_text_output().unwrap_or_default());
    Ok(())
}
```

### Advanced Builder Pattern
```rust
let response = client
    .create_response_builder("gpt-4.1-nano", "Your prompt here")
    .temperature(0.7)
    .max_tokens(200)
    .instructions("System instructions here")
    .with_tools(vec![tool1, tool2])
    .send()
    .await?;
```

### Error Handling
```rust
match client.create_simple_response("gpt-4.1-nano", "Hello").await {
    Ok(response) => println!("Success: {}", response.id),
    Err(OpenAIError::Authentication(msg)) => eprintln!("Auth error: {}", msg),
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
- ✅ Async/await throughout
- ✅ Proper error handling with `thiserror`
- ✅ Type-safe JSON serialization with `serde`
- ✅ Configurable retry logic with `backon`
- ✅ Environment variable configuration
- ✅ Builder pattern API

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