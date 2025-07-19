use openai_responses::OpenAIClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client from environment variables
    let client = OpenAIClient::from_env()?;
    
    // Simple response creation
    let response = client
        .create_simple_response("gpt-4.1-nano", "Write a haiku about Rust")
        .await?;
    
    println!("Response ID: {}", response.id);
    println!("Status: {:?}", response.status);
    
    if let Some(text) = response.get_text_output() {
        println!("Generated text: {}", text);
    }
    
    if let Some(usage) = response.usage {
        println!("Total tokens: {}", usage.total_tokens);
    }
    
    Ok(())
}