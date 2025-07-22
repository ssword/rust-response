use openai_responses::{OpenAIClient, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::from_env()?;

    // Create a response with background processing
    let response = client
        .create_response_builder(Model::Gpt4_1Nano, "Write a detailed tutorial about Rust async programming")
        .temperature(0.7)
        .max_tokens(500)
        .background(true)
        .send()
        .await?;
    
    println!("Response ID: {}", response.id);
    println!("Status: {:?}", response.status);
    
    if response.status == openai_responses::ResponseStatus::InProgress {
        println!("Waiting for response to complete...");
        let completed_response = client.wait_for_response(&response.id).await?;
        
        println!("Response completed!");
        if let Some(text) = completed_response.get_text_output() {
            println!("Generated text: {}", text);
        }
    } else {
        if let Some(text) = response.get_text_output() {
            println!("Generated text: {}", text);
        }
    }
    
    // Clean up the response
    let deleted = client.delete_response(&response.id).await?;
    if deleted.deleted {
        println!("Response deleted successfully");
    }
    
    Ok(())
}