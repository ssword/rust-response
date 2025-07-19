//! # Zero-Copy Parsing with LazyResponse
//!
//! This example demonstrates the LazyResponse type for memory-efficient
//! parsing of large API responses using zero-copy techniques.

use openai_responses::{OpenAIClient, Model, LazyResponse};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::from_env()?;
    
    println!("=== Zero-Copy Parsing with LazyResponse ===\n");
    
    // Example 1: Basic LazyResponse usage
    println!("1. Basic LazyResponse usage:");
    let response = client
        .create_simple_response(Model::Gpt4_1, "Write a detailed explanation of Rust's ownership system")
        .await?;
    
    // Convert to JSON string for lazy parsing demonstration
    let json_str = serde_json::to_string(&response)?;
    println!("Response JSON size: {} bytes", json_str.len());
    
    // Parse with LazyResponse for zero-copy access
    let start = Instant::now();
    let lazy_response: LazyResponse = serde_json::from_str(&json_str)?;
    let parse_time = start.elapsed();
    
    println!("LazyResponse parse time: {:?}", parse_time);
    println!("Response ID: {}", lazy_response.id);
    println!("Status: {:?}", lazy_response.status);
    
    // Check status without parsing output
    if lazy_response.is_completed() {
        println!("✅ Response completed successfully");
        
        // Only parse text when needed
        let start = Instant::now();
        if let Ok(Some(text)) = lazy_response.get_text_output() {
            let text_parse_time = start.elapsed();
            println!("Text extraction time: {:?}", text_parse_time);
            println!("Text preview: {}...", &text[..100.min(text.len())]);
        }
    } else if lazy_response.is_in_progress() {
        println!("⏳ Response still in progress");
    } else if lazy_response.is_cancelled() {
        println!("❌ Response was cancelled");
    }
    
    // Example 2: Memory efficiency comparison
    println!("\n2. Memory efficiency comparison:");
    
    // Generate a larger response for comparison
    let large_response = client
        .create_simple_response(
            Model::Gpt4_1, 
            "Write a comprehensive guide to Rust programming including ownership, borrowing, lifetimes, traits, generics, and async programming with detailed examples"
        )
        .await?;
    
    let large_json = serde_json::to_string(&large_response)?;
    println!("Large response size: {} bytes", large_json.len());
    
    // Standard parsing (full deserialization)
    let start = Instant::now();
    let _standard_response: openai_responses::Response = serde_json::from_str(&large_json)?;
    let standard_time = start.elapsed();
    
    // Lazy parsing (minimal deserialization)
    let start = Instant::now();
    let lazy_large: LazyResponse = serde_json::from_str(&large_json)?;
    let lazy_time = start.elapsed();
    
    println!("Standard parsing time: {:?}", standard_time);
    println!("Lazy parsing time: {:?}", lazy_time);
    println!("Performance improvement: {:.2}x faster", 
             standard_time.as_nanos() as f64 / lazy_time.as_nanos() as f64);
    
    // Example 3: Selective field access
    println!("\n3. Selective field access:");
    
    // Access only the fields you need
    println!("Response ID: {}", lazy_large.id);
    println!("Object type: {}", lazy_large.object);
    
    // Token usage is parsed immediately (small data)
    if let Some(tokens) = lazy_large.get_total_tokens() {
        println!("Total tokens used: {}", tokens);
    }
    
    // Parse metadata only if present and needed
    if let Ok(Some(metadata)) = lazy_large.parse_metadata() {
        println!("Metadata available: {} fields", 
                 metadata.as_object().map(|o| o.len()).unwrap_or(0));
    } else {
        println!("No metadata present");
    }
    
    // Example 4: Conditional parsing based on status
    println!("\n4. Conditional parsing based on status:");
    
    match lazy_large.status {
        openai_responses::ResponseStatus::Completed => {
            println!("✅ Response completed - parsing full output");
            
            let start = Instant::now();
            if let Ok(Some(outputs)) = lazy_large.parse_output() {
                let full_parse_time = start.elapsed();
                println!("Full output parsing time: {:?}", full_parse_time);
                println!("Number of output items: {}", outputs.len());
                
                for (i, output) in outputs.iter().enumerate() {
                    println!("Output {}: {} content items", i + 1, output.content.len());
                }
            }
        },
        openai_responses::ResponseStatus::InProgress => {
            println!("⏳ Response in progress - skipping output parsing");
        },
        openai_responses::ResponseStatus::Incomplete => {
            println!("⚠️  Response incomplete - checking details");
            // Could parse incomplete_details here if needed
        },
        openai_responses::ResponseStatus::Cancelled => {
            println!("❌ Response cancelled - no output to parse");
        },
    }
    
    // Example 5: Error handling with lazy parsing
    println!("\n5. Error handling with lazy parsing:");
    
    // Simulate malformed JSON for error handling
    let malformed_json = r#"{"id": "test", "object": "response", "status": "completed", "output": [invalid_json]}"#;
    
    match serde_json::from_str::<LazyResponse>(malformed_json) {
        Ok(lazy_resp) => {
            println!("LazyResponse created successfully");
            
            // Error will occur when trying to parse the malformed output
            match lazy_resp.parse_output() {
                Ok(Some(outputs)) => {
                    println!("Outputs parsed: {}", outputs.len());
                },
                Ok(None) => {
                    println!("No output present");
                },
                Err(e) => {
                    println!("❌ Error parsing output: {}", e);
                    println!("✅ But we can still access other fields!");
                    println!("ID: {}", lazy_resp.id);
                    println!("Status: {:?}", lazy_resp.status);
                }
            }
        },
        Err(e) => {
            println!("❌ Failed to create LazyResponse: {}", e);
        }
    }
    
    // Example 6: Performance tips
    println!("\n6. Performance tips:");
    println!("💡 Use LazyResponse when:");
    println!("   - Working with large responses (>1MB)");
    println!("   - Only need specific fields (status, ID, tokens)");
    println!("   - Processing many responses in batch");
    println!("   - Memory usage is a concern");
    println!("   - Response content might be malformed");
    
    println!("\n💡 Use standard Response when:");
    println!("   - Working with small responses (<100KB)");
    println!("   - Need to access all fields");
    println!("   - Performing complex content analysis");
    println!("   - Memory usage is not a concern");
    
    Ok(())
}

// Run this example with:
// cargo run --example lazy_response
