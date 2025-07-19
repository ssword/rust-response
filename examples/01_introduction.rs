//! # Introduction to OpenAI Responses API
//!
//! This example demonstrates the basics of using the OpenAI Responses API,
//! including simple text generation and basic response handling.

use openai_responses::OpenAIClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client from environment variables
    // Make sure to set OPENAI_API_KEY in your environment
    let client = OpenAIClient::from_env()?;
    
    println!("=== OpenAI Responses API - Introduction ===\n");
    
    // Example 1: Simple text generation
    println!("1. Simple text generation:");
    let response = client
        .create_simple_response("gpt-4.1-nano", "Explain what Rust is in one sentence")
        .await?;
    
    println!("Response ID: {}", response.id);
    println!("Status: {:?}", response.status);
    if let Some(text) = response.get_text_output() {
        println!("Generated text: {}\n", text);
    }
    
    // Example 2: Using the builder pattern for more control
    println!("2. Using builder pattern:");
    let response = client
        .create_response_builder("gpt-4.1-nano", "Write a haiku about programming")
        .temperature(0.8)
        .max_tokens(50)
        .send()
        .await?;
    
    println!("Haiku: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 3: Basic error handling
    println!("3. Basic error handling:");
    match client.create_simple_response("gpt-4.1-nano", "Hello").await {
        Ok(response) => println!("✅ Success! Response ID: {}", response.id),
        Err(e) => println!("❌ Error: {}", e),
    }
    
    Ok(())
}

// Run this example with:
// cargo run --example 01_introduction