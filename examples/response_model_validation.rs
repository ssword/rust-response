//! # Compile-Time Model Validation with TypedModel
//!
//! This example demonstrates the TypedModel system for compile-time
//! validation of model capabilities and type-safe request building.

use openai_responses::{
    OpenAIClient, Model, CreateResponseRequest, 
    ReasoningEffort, Modality, ValidationError
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::from_env()?;
    
    println!("=== Compile-Time Model Validation ===\n");
    
    // Example 1: Model capability checking
    println!("1. Model capability checking:");
    
    let models = [
        Model::Gpt4_1,
        Model::Gpt4_1Mini,
        Model::Gpt4_1Nano,
        Model::Gpt4O,
        Model::Gpt4OMini,
        Model::O1,
        Model::O3Mini,
        Model::O4Mini,
    ];
    
    for model in &models {
        println!("Model: {}", model.as_str());
        println!("  Reasoning: {}", if model.supports_reasoning() { "✅" } else { "❌" });
        println!("  Vision: {}", if model.supports_vision() { "✅" } else { "❌" });
        println!("  JSON Mode: {}", if model.supports_json_mode() { "✅" } else { "❌" });
        println!("  Function Calling: {}", if model.supports_function_calling() { "✅" } else { "❌" });
        println!("  Context Window: {} tokens", model.max_context_window());
        println!();
    }
    
    // Example 2: Safe request building with validation
    println!("2. Safe request building with validation:");
    
    // This will work - O1 supports reasoning
    let reasoning_model = Model::O1;
    if reasoning_model.supports_reasoning() {
        let request = CreateResponseRequest::new(
            reasoning_model,
            "Solve this complex mathematical problem step by step"
        )
        .with_reasoning(ReasoningEffort::High);
        
        match request.validate_for_model() {
            Ok(()) => println!("✅ Reasoning request validated successfully"),
            Err(e) => println!("❌ Validation failed: {}", e),
        }
    }
    
    // This will fail validation - O1 doesn't support vision
    let vision_request = CreateResponseRequest::new(
        Model::O1,
        "Analyze this image"
    )
    .with_modalities(vec![Modality::Image]);
    
    match vision_request.validate_for_model() {
        Ok(()) => println!("✅ Vision request validated"),
        Err(ValidationError::VisionNotSupported(model)) => {
            println!("❌ Expected validation failure: {} doesn't support vision", model.as_str());
        },
        Err(e) => println!("❌ Unexpected validation error: {}", e),
    }
    
    // Example 3: Model filtering by capabilities
    println!("\n3. Model filtering by capabilities:");
    
    use openai_responses::ModelCapability;
    
    let reasoning_models = Model::with_capability(ModelCapability::Reasoning);
    println!("Reasoning-capable models: {:?}", reasoning_models);
    
    let vision_models = Model::with_capability(ModelCapability::Vision);
    println!("Vision-capable models: {:?}", vision_models);
    
    let json_models = Model::with_capability(ModelCapability::JsonMode);
    println!("JSON-capable models: {:?}", json_models);
    
    let function_models = Model::with_capability(ModelCapability::FunctionCalling);
    println!("Function-calling models: {:?}", function_models);
    
    // Example 4: Dynamic model selection
    println!("\n4. Dynamic model selection:");
    
    fn select_best_model_for_task(task_type: &str) -> Model {
        match task_type {
            "reasoning" => {
                let reasoning_models = Model::with_capability(ModelCapability::Reasoning);
                reasoning_models.into_iter()
                    .max_by_key(|m| m.max_context_window())
                    .unwrap_or(Model::Gpt4_1)
            },
            "vision" => {
                let vision_models = Model::with_capability(ModelCapability::Vision);
                vision_models.into_iter()
                    .max_by_key(|m| m.max_context_window())
                    .unwrap_or(Model::Gpt4_1)
            },
            "json" => {
                let json_models = Model::with_capability(ModelCapability::JsonMode);
                json_models.into_iter()
                    .min_by_key(|m| m.max_context_window()) // Use smallest for simple JSON tasks
                    .unwrap_or(Model::Gpt4_1Nano)
            },
            _ => Model::Gpt4_1Nano, // Default for general tasks
        }
    }
    
    let reasoning_model = select_best_model_for_task("reasoning");
    let vision_model = select_best_model_for_task("vision");
    let json_model = select_best_model_for_task("json");
    
    println!("Best reasoning model: {} ({})", reasoning_model.as_str(), reasoning_model.max_context_window());
    println!("Best vision model: {} ({})", vision_model.as_str(), vision_model.max_context_window());
    println!("Best JSON model: {} ({})", json_model.as_str(), json_model.max_context_window());
    
    // Example 5: Validation error handling
    println!("\n5. Validation error handling:");
    
    let test_cases = vec![
        ("Temperature too high", || {
            CreateResponseRequest::new(Model::Gpt4_1Nano, "Test")
                .with_temperature(3.0) // Invalid: > 2.0
        }),
        ("Top-p too high", || {
            CreateResponseRequest::new(Model::Gpt4_1Nano, "Test")
                .with_top_p(1.5) // Invalid: > 1.0
        }),
        ("Frequency penalty too low", || {
            CreateResponseRequest::new(Model::Gpt4_1Nano, "Test")
                .with_frequency_penalty(-3.0) // Invalid: < -2.0
        }),
        ("Reasoning on non-reasoning model", || {
            CreateResponseRequest::new(Model::Gpt4_1Nano, "Test")
                .with_reasoning(ReasoningEffort::High) // Invalid: model doesn't support reasoning
        }),
    ];
    
    for (test_name, create_request) in test_cases {
        let request = create_request();
        match request.validate_for_model() {
            Ok(()) => println!("❌ {}: Expected validation to fail", test_name),
            Err(e) => println!("✅ {}: Correctly caught error - {}", test_name, e),
        }
    }
    
    // Example 6: Builder pattern with validation
    println!("\n6. Builder pattern with validation:");
    
    // Using the builder for safe construction
    let builder_result = CreateResponseRequest::builder(Model::Gpt4_1, "Analyze this data")
        .temperature(0.7)
        .and_then(|b| b.max_tokens(200))
        .and_then(|b| b.instructions("Be thorough and accurate"))
        .and_then(|b| b.build());
    
    match builder_result {
        Ok(request) => {
            println!("✅ Builder request created successfully");
            println!("   Model: {}", request.model.as_str());
            println!("   Temperature: {:?}", request.temperature);
            println!("   Max tokens: {:?}", request.max_tokens);
        },
        Err(e) => println!("❌ Builder failed: {}", e),
    }
    
    // Example 7: Context window management
    println!("\n7. Context window management:");
    
    fn estimate_tokens(text: &str) -> usize {
        // Rough estimation: ~4 characters per token
        text.len() / 4
    }
    
    fn check_context_fit(model: Model, input: &str, max_output: usize) -> bool {
        let input_tokens = estimate_tokens(input);
        let total_needed = input_tokens + max_output;
        total_needed <= model.max_context_window()
    }
    
    let long_input = "This is a very long input that might exceed context limits...".repeat(1000);
    let max_output_tokens = 1000;
    
    for model in &[Model::Gpt4_1Nano, Model::Gpt4_1, Model::O1] {
        let fits = check_context_fit(*model, &long_input, max_output_tokens);
        println!("{}: {} ({})", 
                 model.as_str(),
                 if fits { "✅ Fits" } else { "❌ Too large" },
                 model.max_context_window());
    }
    
    // Example 8: Feature compatibility matrix
    println!("\n8. Feature compatibility matrix:");
    
    struct FeatureRequest {
        reasoning: bool,
        vision: bool,
        json_mode: bool,
        function_calling: bool,
    }
    
    fn find_compatible_models(features: &FeatureRequest) -> Vec<Model> {
        Model::all().iter()
            .filter(|model| {
                (!features.reasoning || model.supports_reasoning()) &&
                (!features.vision || model.supports_vision()) &&
                (!features.json_mode || model.supports_json_mode()) &&
                (!features.function_calling || model.supports_function_calling())
            })
            .copied()
            .collect()
    }
    
    let feature_sets = vec![
        ("Basic text", FeatureRequest { reasoning: false, vision: false, json_mode: false, function_calling: false }),
        ("JSON output", FeatureRequest { reasoning: false, vision: false, json_mode: true, function_calling: false }),
        ("Vision + JSON", FeatureRequest { reasoning: false, vision: true, json_mode: true, function_calling: false }),
        ("Reasoning only", FeatureRequest { reasoning: true, vision: false, json_mode: false, function_calling: false }),
        ("All features", FeatureRequest { reasoning: true, vision: true, json_mode: true, function_calling: true }),
    ];
    
    for (name, features) in feature_sets {
        let compatible = find_compatible_models(&features);
        println!("{}: {} models", name, compatible.len());
        for model in compatible {
            println!("  - {}", model.as_str());
        }
    }
    
    println!("\n💡 Type safety benefits:");
    println!("   - Compile-time capability checking prevents runtime errors");
    println!("   - Model enum ensures only valid models are used");
    println!("   - Validation catches configuration errors early");
    println!("   - Builder pattern provides ergonomic error handling");
    println!("   - Context window checking prevents token limit errors");
    
    Ok(())
}

// Run this example with:
// cargo run --example typed_model_validation
