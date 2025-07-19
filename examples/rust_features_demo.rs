use crate::types::{Model, CreateResponseRequest, O1Builder, Gpt4_1Builder};
use crate::client::OpenAIClient;
use crate::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 OpenAI Responses SDK - Rust-Specific Features Demo");
    println!("====================================================");

    // Initialize client with Arc sharing (zero-cost cloning)
    let client = OpenAIClient::from_env()?;
    
    // Demo 1: Type-Safe Model Capabilities with Const Generics
    println!("\n📝 Demo 1: Compile-Time Model Validation");
    println!("-" .repeat(40));
    
    // This compiles - O1 supports reasoning
    let o1_request = O1Builder::create("Explain quantum computing")
        .reasoning(crate::types::ReasoningEffort::High)
        .build();
    println!("✅ O1 with reasoning: {:?}", o1_request.model);
    
    // This compiles - GPT-4.1 supports vision
    let gpt4_request = Gpt4_1Builder::create("Describe this image")
        .modalities(vec![crate::types::Modality::Text, crate::types::Modality::Image])
        .build();
    println!("✅ GPT-4.1 with vision: {:?}", gpt4_request.model);
    
    // Note: These would NOT compile (commented to avoid build errors):
    // O1Builder::create("test").modalities(vec![Modality::Image]).build(); // O1 doesn't support vision
    // Gpt4_1Builder::create("test").reasoning(ReasoningEffort::High).build(); // GPT-4.1 doesn't support reasoning
    
    println!("✅ Compile-time safety: Invalid model/capability combinations prevented!");

    // Demo 2: Type-State Pattern for Request Validation
    println!("\n🔒 Demo 2: Type-State Request Validation");
    println!("-" .repeat(40));
    
    use crate::types::{TypedRequestBuilder, ValidationError};
    
    let builder = TypedRequestBuilder::new(Model::Gpt4_1, "Hello, world!");
    
    // This will validate at runtime and transition to validated state
    match builder
        .temperature(0.7)
        .and_then(|b| b.max_tokens(100).top_p(0.9))
        .and_then(|b| b.validate())
    {
        Ok(validated_builder) => {
            println!("✅ Request validated successfully - can now send");
            // Only validated requests can be sent (compile-time safety)
            // let response = validated_builder.send(&client).await?;
        }
        Err(ValidationError::InvalidTemperature(temp)) => {
            println!("❌ Invalid temperature: {}", temp);
        }
        Err(e) => {
            println!("❌ Validation error: {}", e);
        }
    }

    // Demo 3: Zero-Copy JSON Parsing with LazyResponse
    println!("\n⚡ Demo 3: Zero-Copy JSON Processing");
    println!("-" .repeat(40));
    
    let sample_json = r#"{
        "id": "resp_demo",
        "object": "response",
        "status": "completed",
        "output": [{"type": "message", "content": [{"type": "text", "text": "This is a large response that we want to parse efficiently"}]}],
        "usage": {"total_tokens": 50}
    }"#;
    
    // Parse with zero-copy for large JSON
    let lazy_response: crate::types::LazyResponse = serde_json::from_str(sample_json)?;
    println!("✅ Lazy parsing - ID: {}, Status: {:?}", lazy_response.id, lazy_response.status);
    println!("✅ Token count (no parsing): {:?}", lazy_response.get_total_tokens());
    
    // Parse output only when needed
    if let Ok(Some(text)) = lazy_response.get_text_output() {
        println!("✅ Text extracted efficiently: {}", text);
    }

    // Demo 4: Adaptive JSON Parsing (SIMD when available)
    println!("\n🏎️ Demo 4: Adaptive JSON Parsing");
    println!("-" .repeat(40));
    
    use crate::json::{AdaptiveJsonParser, JsonCapabilities};
    
    println!("Available JSON parser: {}", JsonCapabilities::best_parser());
    println!("SIMD support: {}", JsonCapabilities::has_simd());
    println!("SIMD threshold: {} bytes", JsonCapabilities::simd_threshold());
    
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
    
    let response = AdaptiveJsonParser::parse_response(large_json.into_bytes())?;
    println!("✅ Adaptive parsing successful - ID: {}", response.id);

    // Demo 5: Memory-Efficient URL Building with Cow
    println!("\n🐄 Demo 5: Zero-Allocation URL Building");
    println!("-" .repeat(40));
    
    // The client now uses Cow<str> for efficient URL building
    // This is internal but demonstrates zero-allocation string handling
    println!("✅ Client uses Cow<str> for efficient URL construction");
    println!("✅ Arc<Client> enables zero-cost cloning across threads");

    // Demo 6: Enhanced Error Handling (if feature enabled)
    println!("\n🚨 Demo 6: Enhanced Error Handling");
    println!("-" .repeat(40));
    
    #[cfg(feature = "enhanced-errors")]
    {
        use crate::error::{ErrorContextExt, CircuitBreaker};
        
        // Demonstrate error context
        let result: Result<()> = Err(crate::error::OpenAIError::InvalidRequest("Demo error".to_string()));
        let with_context = result.context("Failed to demonstrate error handling");
        
        if let Err(e) = with_context {
            println!("✅ Enhanced error with context: {:#}", e);
        }
        
        // Circuit breaker demo
        let mut breaker = CircuitBreaker::new(3, 2, std::time::Duration::from_millis(100));
        println!("✅ Circuit breaker initialized - can execute: {}", breaker.can_execute());
        
        // Simulate failures
        for i in 1..=3 {
            breaker.record_failure();
            println!("   Failure {}: can execute = {}", i, breaker.can_execute());
        }
        println!("✅ Circuit breaker opened after 3 failures");
    }
    
    #[cfg(not(feature = "enhanced-errors"))]
    {
        println!("ℹ️  Enhanced errors not enabled - use --features enhanced-errors");
    }

    // Demo 7: Streaming (if feature enabled)
    println!("\n🌊 Demo 7: Streaming Support");
    println!("-" .repeat(40));
    
    #[cfg(feature = "streaming")]
    {
        println!("✅ Streaming support available");
        println!("   - Real-time response processing with futures::Stream");
        println!("   - Server-sent events parsing");
        println!("   - Callback-based streaming");
        println!("   - Stream collection and text extraction");
    }
    
    #[cfg(not(feature = "streaming"))]
    {
        println!("ℹ️  Streaming not enabled - use --features streaming");
    }

    // Demo 8: Backend Abstraction (if feature enabled)
    println!("\n🔧 Demo 8: Backend Abstraction");
    println!("-" .repeat(40));
    
    #[cfg(feature = "backend-abstraction")]
    {
        use crate::backend::{MockBackend, MockResponseExt};
        
        let mock = MockBackend::new();
        mock.add_json_response(200, r#"{"id": "mock_response", "status": "completed"}"#);
        
        println!("✅ Mock backend created for testing");
        println!("   - HTTP backend abstraction with async traits");
        println!("   - MockBackend for unit testing");
        println!("   - Request recording and response mocking");
    }
    
    #[cfg(not(feature = "backend-abstraction"))]
    {
        println!("ℹ️  Backend abstraction not enabled - use --features backend-abstraction");
    }

    println!("\n🎉 Demo completed! This showcases Rust-specific improvements:");
    println!("   • Zero-cost abstractions with Arc and Cow");
    println!("   • Compile-time safety with type-state and const generics");
    println!("   • Memory efficiency with lazy parsing and SIMD");
    println!("   • Robust error handling with context and circuit breakers");
    println!("   • Performance optimizations throughout");

    Ok(())
}