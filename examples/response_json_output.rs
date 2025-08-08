//! # Structured Output with JSON Schema
//!
//! This example demonstrates how to generate structured, validated JSON responses
//! using the Responses API with JSON schema validation.

use openai_responses::{OpenAIClient, Model};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    email: String,
    skills: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Recipe {
    name: String,
    ingredients: Vec<Ingredient>,
    instructions: Vec<String>,
    prep_time_minutes: u32,
    difficulty: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ingredient {
    name: String,
    amount: String,
    unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnalysisResult {
    sentiment: String,
    confidence: f64,
    key_topics: Vec<String>,
    summary: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::from_env()?;
    
    println!("=== Structured Output with JSON Schema ===\n");
    
    // Check if the model supports JSON mode
    let model = Model::Gpt4_1Nano;
    if !model.supports_json_mode() {
        println!("⚠️  Model {} doesn't support JSON mode. Using a compatible model instead.", model.as_str());
    }

    // Example 1: Person profile generation
    println!("1. Person profile generation:");
    let schema = r#"{
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age": {"type": "integer", "minimum": 18, "maximum": 100},
            "email": {"type": "string", "format": "email"},
            "skills": {
                "type": "array",
                "items": {"type": "string"},
                "minItems": 1,
                "maxItems": 10
            }
        },
        "required": ["name", "age", "email", "skills"]
    }"#;

    let response = client
        .create_response_builder(model, "Generate a profile for a senior Rust developer")
        .instructions(&format!("Return valid JSON matching this schema: {}", schema))
        .max_tokens(200)
        .send()
        .await?;
    
    if let Some(json_text) = response.get_text_output() {
        let person: Person = serde_json::from_str(&json_text)?;
        println!("Generated person: {:?}\n", person);
    }
    
    // Example 2: Recipe generation
    println!("2. Recipe generation with structure:");
    let recipe_schema = r#"{
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "ingredients": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"},
                        "amount": {"type": "string"},
                        "unit": {"type": "string"}
                    },
                    "required": ["name", "amount", "unit"]
                }
            },
            "instructions": {
                "type": "array",
                "items": {"type": "string"}
            },
            "prep_time_minutes": {"type": "integer", "minimum": 1, "maximum": 300},
            "difficulty": {"type": "string", "enum": ["easy", "medium", "hard"]}
        },
        "required": ["name", "ingredients", "instructions", "prep_time_minutes", "difficulty"]
    }"#;
    
    let response = client
        .create_response_builder(Model::Gpt4_1Nano, "Create a recipe for chocolate chip cookies")
        .instructions(&format!("Return valid JSON matching this schema: {}", recipe_schema))
        .max_tokens(300)
        .send()
        .await?;
    
    if let Some(json_text) = response.get_text_output() {
        let recipe: Recipe = serde_json::from_str(&json_text)?;
        println!("Recipe: {}", recipe.name);
        println!("Prep time: {} minutes", recipe.prep_time_minutes);
        println!("Difficulty: {}", recipe.difficulty);
        println!("Ingredients: {} items", recipe.ingredients.len());
    }
    
    // Example 3: Sentiment analysis
    println!("\n3. Sentiment analysis with structured output:");
    let sentiment_schema = r#"{
        "type": "object",
        "properties": {
            "sentiment": {"type": "string", "enum": ["positive", "negative", "neutral"]},
            "confidence": {"type": "number", "minimum": 0, "maximum": 1},
            "key_topics": {
                "type": "array",
                "items": {"type": "string"},
                "maxItems": 5
            },
            "summary": {"type": "string", "maxLength": 100}
        },
        "required": ["sentiment", "confidence", "key_topics", "summary"]
    }"#;
    
    let text = "I absolutely love the new Rust async features! The performance improvements are incredible, though the learning curve was a bit steep at first.";
    let response = client
        .create_response_builder(Model::Gpt4_1Nano, &format!("Analyze this text: {}", text))
        .instructions(&format!("Return valid JSON matching this schema: {}", sentiment_schema))
        .max_tokens(150)
        .send()
        .await?;
    
    if let Some(json_text) = response.get_text_output() {
        let analysis: AnalysisResult = serde_json::from_str(&json_text)?;
        println!("Sentiment: {} (confidence: {:.2})", analysis.sentiment, analysis.confidence);
        println!("Key topics: {}", analysis.key_topics.join(", "));
        println!("Summary: {}", analysis.summary);
    }
    
    // Example 4: Batch processing
    println!("\n4. Batch processing with validation:");
    
    let items = vec!["Apple", "Banana", "Carrot"];
    let schema = r#"{
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "category": {"type": "string", "enum": ["fruit", "vegetable"]},
            "calories_per_100g": {"type": "integer", "minimum": 0, "maximum": 500},
            "color": {"type": "string"}
        },
        "required": ["name", "category", "calories_per_100g", "color"]
    }"#;
    
    let mut results = Vec::new();
    
    for item in items {
        let response = client
            .create_response_builder("gpt-4.1-nano", &format!("Classify {} as food", item))
            .instructions(&format!("Return valid JSON matching this schema: {}", schema))
            .max_tokens(100)
            .send()
            .await?;
        
        if let Some(json_text) = response.get_text_output() {
            match serde_json::from_str::<serde_json::Value>(&json_text) {
                Ok(parsed) => results.push(parsed),
                Err(e) => println!("Failed to parse JSON for {}: {}", item, e),
            }
        }
    }
    
    println!("Batch results: {}\n", serde_json::to_string_pretty(&results)?);
    
    // Example 5: Nested structures
    println!("5. Complex nested structures:");
    let company_schema = r#"{
        "type": "object",
        "properties": {
            "company": {
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "founded": {"type": "integer", "minimum": 1900, "maximum": 2024},
                    "employees": {"type": "integer", "minimum": 1},
                    "departments": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "name": {"type": "string"},
                                "headcount": {"type": "integer", "minimum": 1},
                                "budget": {"type": "number", "minimum": 0}
                            },
                            "required": ["name", "headcount", "budget"]
                        }
                    }
                },
                "required": ["name", "founded", "employees", "departments"]
            }
        },
        "required": ["company"]
    }"#;
    
    let response = client
        .create_response_builder("gpt-4.1-nano", "Create a tech company profile")
        .instructions(&format!("Return valid JSON matching this schema: {}", company_schema))
        .max_tokens(400)
        .send()
        .await?;
    
    if let Some(json_text) = response.get_text_output() {
        let company_data: serde_json::Value = serde_json::from_str(&json_text)?;
        println!("Company data: {}\n", serde_json::to_string_pretty(&company_data)?);
    }
    
    Ok(())
}

// Run this example with:
// cargo run --example response_json_output
