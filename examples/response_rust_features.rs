use openai_responses::{
    OpenAIClient, Model, CreateResponseRequest,
    ReasoningEffort, Modality, LazyResponse
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 OpenAI Responses SDK - Rust-Specific Features Demo");
    println!("====================================================");

    // Initialize client with Arc sharing (zero-cost cloning)
    let client = OpenAIClient::from_env()?;
    
    // Demo 1: Type-Safe Model Capabilities
    println!("\n📝 Demo 1: Model Capability Validation");
    println!("-".repeat(40));

    // Check model capabilities at runtime
    let o1_model = Model::O1;
    let gpt4_model = Model::Gpt4_1;

    println!("Model capabilities:");
    println!("  O1 - Reasoning: {}, Vision: {}",
             o1_model.supports_reasoning(), o1_model.supports_vision());
    println!("  GPT-4.1 - Reasoning: {}, Vision: {}",
             gpt4_model.supports_reasoning(), gpt4_model.supports_vision());

    // Create requests with capability checking
    if o1_model.supports_reasoning() {
        let o1_request = CreateResponseRequest::new(o1_model, "Explain quantum computing")
            .with_reasoning(ReasoningEffort::High);
        println!("✅ O1 with reasoning: {}", o1_request.model.as_str());
    }

    if gpt4_model.supports_vision() {
        let gpt4_request = CreateResponseRequest::new(gpt4_model, "Describe this image")
            .with_modalities(vec![Modality::Text, Modality::Image]);
        println!("✅ GPT-4.1 with vision: {}", gpt4_request.model.as_str());
    }

    println!("✅ Runtime safety: Model capabilities validated before use!");

    // Demo 2: Request Validation
    println!("\n🔒 Demo 2: Request Validation");
    println!("-".repeat(40));

    use openai_responses::ValidationError;

    // Create a request and validate it
    let request = CreateResponseRequest::new(Model::Gpt4_1, "Hello, world!")
        .with_temperature(0.7)
        .with_max_tokens(100);

    match request.validate_for_model() {
        Ok(()) => {
            println!("✅ Request validated successfully");
        }
        Err(e) => {
            println!("❌ Validation error: {}", e);
        }
    }

    // Test validation with invalid parameters
    let invalid_request = CreateResponseRequest::new(Model::O1, "Test")
        .with_modalities(vec![Modality::Image]); // O1 doesn't support vision

    match invalid_request.validate_for_model() {
        Ok(()) => {
            println!("❌ Should have failed validation");
        }
        Err(ValidationError::VisionNotSupported(model)) => {
            println!("✅ Correctly caught vision error for {}", model.as_str());
        }
        Err(e) => {
            println!("❌ Unexpected validation error: {}", e);
        }
    }

    // Demo 3: Zero-Copy JSON Parsing with LazyResponse
    println!("\n⚡ Demo 3: Zero-Copy JSON Processing");
    println!("-".repeat(40));

    let sample_json = r#"{
        "id": "resp_demo",
        "object": "response",
        "status": "completed",
        "output": [{"type": "message", "content": [{"type": "text", "text": "This is a large response that we want to parse efficiently"}]}],
        "usage": {"total_tokens": 50}
    }"#;

    // Parse with zero-copy for large JSON
    let lazy_response: LazyResponse = serde_json::from_str(sample_json)?;
    println!("✅ Lazy parsing - ID: {}, Status: {:?}", lazy_response.id, lazy_response.status);
    println!("✅ Token count (no parsing): {:?}", lazy_response.get_total_tokens());

    // Parse output only when needed
    if let Ok(Some(text)) = lazy_response.get_text_output() {
        println!("✅ Text extracted efficiently: {}", text);
    }

    // Demo 4: Performance Comparison
    println!("\n🏎️ Demo 4: Performance Comparison");
    println!("-".repeat(40));

    use std::time::Instant;
    use openai_responses::Response;

    let large_json = format!(r#"{{
        "id": "resp_large",
        "object": "response",
        "status": "completed",
        "output": [{{
            "type": "message",
            "content": [{{
                "type": "text",
                "text": "{}"
            }}]
        }}],
        "usage": {{"total_tokens": 1000}}
    }}"#, "Large response content ".repeat(100));

    println!("JSON size: {} bytes", large_json.len());

    // Standard parsing
    let start = Instant::now();
    let _standard_response: Response = serde_json::from_str(&large_json)?;
    let standard_time = start.elapsed();

    // Lazy parsing
    let start = Instant::now();
    let _lazy_response: LazyResponse = serde_json::from_str(&large_json)?;
    let lazy_time = start.elapsed();

    println!("✅ Standard parsing: {:?}", standard_time);
    println!("✅ Lazy parsing: {:?}", lazy_time);
    println!("✅ Performance improvement: {:.2}x",
             standard_time.as_nanos() as f64 / lazy_time.as_nanos() as f64);

    // Demo 5: Model Enum Benefits
    println!("\n🐄 Demo 5: Type-Safe Model Selection");
    println!("-".repeat(40));

    // Model enum provides compile-time safety
    let all_models = Model::all();
    println!("✅ Available models: {}", all_models.len());

    for model in all_models {
        println!("  - {} (context: {})", model.as_str(), model.max_context_window());
    }

    // Model filtering
    use openai_responses::ModelCapability;
    let reasoning_models = Model::with_capability(ModelCapability::Reasoning);
    println!("✅ Reasoning models: {:?}", reasoning_models);

    // Demo 6: Error Handling
    println!("\n🚨 Demo 6: Error Handling");
    println!("-".repeat(40));

    // Demonstrate validation errors
    let invalid_temp_request = CreateResponseRequest::new(Model::Gpt4_1Nano, "Test")
        .with_temperature(3.0); // Invalid temperature

    match invalid_temp_request.validate_for_model() {
        Ok(()) => println!("❌ Should have failed"),
        Err(ValidationError::InvalidTemperature(temp)) => {
            println!("✅ Caught invalid temperature: {}", temp);
        }
        Err(e) => println!("❌ Unexpected error: {}", e),
    }

    // Demonstrate capability errors
    let invalid_capability_request = CreateResponseRequest::new(Model::Gpt4_1Nano, "Test")
        .with_reasoning(ReasoningEffort::High); // GPT-4.1-nano doesn't support reasoning

    match invalid_capability_request.validate_for_model() {
        Ok(()) => println!("❌ Should have failed"),
        Err(ValidationError::ReasoningNotSupported(model)) => {
            println!("✅ Caught unsupported reasoning for: {}", model.as_str());
        }
        Err(e) => println!("❌ Unexpected error: {}", e),
    }

    // Demo 7: Serialization and Deserialization
    println!("\n🌊 Demo 7: Serialization Features");
    println!("-".repeat(40));

    // Model serialization
    let model = Model::Gpt4_1Nano;
    let model_json = serde_json::to_string(&model)?;
    println!("✅ Model serialized: {}", model_json);

    let deserialized_model: Model = serde_json::from_str(&model_json)?;
    println!("✅ Model deserialized: {}", deserialized_model.as_str());

    // Request serialization
    let request = CreateResponseRequest::new(Model::Gpt4_1, "Test request")
        .with_temperature(0.7)
        .with_max_tokens(100);

    let request_json = serde_json::to_string_pretty(&request)?;
    println!("✅ Request serialized successfully ({} bytes)", request_json.len());

    // Demo 8: Real API Usage
    println!("\n🔧 Demo 8: Real API Usage");
    println!("-".repeat(40));

    // Make a real API call (if API key is available)
    match std::env::var("OPENAI_API_KEY") {
        Ok(_) => {
            println!("✅ API key found - making real request");
            let response = client
                .create_simple_response(Model::Gpt4_1Nano, "Say hello in Rust style!")
                .await?;

            println!("✅ Response ID: {}", response.id);
            println!("✅ Status: {:?}", response.status);
            if let Some(text) = response.get_text_output() {
                println!("✅ Response: {}", text);
            }
        }
        Err(_) => {
            println!("ℹ️  No API key found - skipping real request");
            println!("   Set OPENAI_API_KEY environment variable to test real API calls");
        }
    }

    println!("\n🎉 Demo completed! This showcases Rust-specific features:");
    println!("   • Type-safe model selection with enum variants");
    println!("   • Runtime capability validation with clear error messages");
    println!("   • Zero-copy JSON parsing with LazyResponse");
    println!("   • Memory-efficient parsing for large responses");
    println!("   • Comprehensive error handling with ValidationError");
    println!("   • Serde integration for serialization/deserialization");
    println!("   • Performance optimizations throughout the API");

    Ok(())
}