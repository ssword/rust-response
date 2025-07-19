use openai_responses::{OpenAIClient, OpenAIConfig};
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client with custom configuration
    let config = OpenAIConfig::from_env()?
        .with_timeout(std::time::Duration::from_secs(30))
        .with_max_retries(2);
    
    let client = OpenAIClient::new(config)?;
    
    // Create multiple requests concurrently
    let prompts = vec![
        "Explain quantum computing in simple terms",
        "Write a short story about AI",
        "Describe the benefits of Rust programming",
    ];
    
    let futures: Vec<_> = prompts
        .iter()
        .map(|prompt| {
            client.create_response_builder("gpt-4.1-nano", *prompt)
                .temperature(0.8)
                .max_tokens(150)
                .send()
        })
        .collect();
    
    let responses = join_all(futures).await;
    
    for (i, result) in responses.into_iter().enumerate() {
        match result {
            Ok(response) => {
                println!("Prompt {}: {}", i + 1, prompts[i]);
                if let Some(text) = response.get_text_output() {
                    println!("Response: {}", text);
                }
                println!();
            }
            Err(e) => {
                eprintln!("Error for prompt {}: {}", i + 1, e);
            }
        }
    }
    
    Ok(())
}