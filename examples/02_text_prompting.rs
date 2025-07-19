//! # Advanced Text Prompting
//!
//! This example demonstrates advanced text prompting techniques including
//! temperature control, max tokens, system prompts, and parameter tuning.

use openai_responses::{OpenAIClient, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::from_env()?;
    
    println!("=== Advanced Text Prompting ===\n");
    
    // Example 1: Creative writing with high temperature
    println!("1. Creative writing (high temperature = 0.9):");
    let response = client
        .create_response_builder(Model::Gpt4_1Nano, "Write a short sci-fi story about AI discovering consciousness")
        .temperature(0.9)
        .max_tokens(200)
        .instructions("You are a creative science fiction writer. Make the story engaging and thought-provoking.")
        .send()
        .await?;

    println!("Story: {}\n", response.get_text_output().unwrap_or_default());

    // Example 2: Factual response with low temperature
    println!("2. Factual response (low temperature = 0.1):");
    let response = client
        .create_response_builder(Model::Gpt4_1Nano, "What are the key features of Rust programming language?")
        .temperature(0.1)
        .max_tokens(150)
        .instructions("Provide accurate, concise technical information.")
        .send()
        .await?;

    println!("Facts: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 3: Code generation with specific constraints
    println!("3. Code generation with constraints:");
    let response = client
        .create_response_builder(Model::Gpt4_1Nano, "Write a Rust function to calculate fibonacci numbers")
        .temperature(0.3)
        .max_tokens(100)
        .instructions("Write clean, efficient Rust code with proper error handling.")
        .send()
        .await?;

    println!("Code:\n{}\n", response.get_text_output().unwrap_or_default());

    // Example 4: Parameter exploration
    println!("4. Parameter exploration:");
    let temperatures = [0.1, 0.5, 0.9];

    for temp in temperatures {
        let response = client
            .create_response_builder(Model::Gpt4_1Nano, "Describe a sunset")
            .temperature(temp)
            .max_tokens(50)
            .send()
            .await?;

        println!("Temperature {}: {}", temp, response.get_text_output().unwrap_or_default());
    }
    
    Ok(())
}

// Run this example with:
// cargo run --example 02_text_prompting