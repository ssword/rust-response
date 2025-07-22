//! # Conversation State Management
//!
//! This example demonstrates how to maintain conversation context
//! across multiple turns using the Responses API.

use openai_responses::{OpenAIClient, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::from_env()?;
    
    println!("=== Conversation State Management ===\n");
    
    // Example 1: Multi-turn conversation with context
    println!("1. Multi-turn conversation:");
    
    let mut conversation_history = vec![];
    
    // Turn 1: Initial greeting
    let response1 = client
        .create_response_builder(Model::Gpt4_1Nano, "Hello! I'm learning Rust programming.")
        .instructions("You are a helpful Rust programming tutor. Be encouraging and provide practical examples.")
        .send()
        .await?;
    
    let reply1 = response1.get_text_output().unwrap_or_default();
    println!("Tutor: {}", reply1);
    conversation_history.push(("user", "Hello! I'm learning Rust programming."));
    conversation_history.push(("assistant", &reply1));
    
    // Turn 2: Follow-up question
    let context = format!("Previous conversation: {:?}\n\nUser: What are the main benefits of Rust?", conversation_history);
    let response2 = client
        .create_response_builder(Model::Gpt4_1Nano, "What are the main benefits of Rust?")
        .instructions(&format!("Previous context: {}\nContinue the conversation naturally.", context))
        .send()
        .await?;
    
    let reply2 = response2.get_text_output().unwrap_or_default();
    println!("Tutor: {}", reply2);
    conversation_history.push(("user", "What are the main benefits of Rust?"));
    conversation_history.push(("assistant", &reply2));
    
    // Turn 3: Specific technical question
    let context = format!("Full conversation history: {:?}\n\nUser: Can you show me a simple ownership example?", conversation_history);
    let response3 = client
        .create_response_builder(Model::Gpt4_1Nano, "Can you show me a simple ownership example?")
        .instructions(&format!("Based on the previous discussion: {}\nProvide a clear ownership example.", context))
        .max_tokens(200)
        .send()
        .await?;
    
    println!("Tutor: {}", response3.get_text_output().unwrap_or_default());
    
    // Example 2: Role-based conversation
    println!("\n2. Role-based conversation:");
    
    let roles = vec![
        ("system", "You are a senior Rust developer mentoring a junior developer."),
        ("user", "I keep getting ownership errors."),
        ("assistant", "Let's work through this together. Can you share the specific error message?"),
        ("user", "It says 'value moved here' when I try to use a variable after passing it to a function."),
    ];
    
    let conversation_prompt = roles
        .iter()
        .map(|(role, content)| format!("{}: {}", role, content))
        .collect::<Vec<_>>()
        .join("\n");
    
    let response = client
        .create_response_builder(Model::Gpt4_1Nano, &conversation_prompt)
        .instructions("Continue this mentoring conversation naturally.")
        .send()
        .await?;
    
    println!("Senior Dev: {}", response.get_text_output().unwrap_or_default());
    
    // Example 3: Session management simulation
    println!("\n3. Session management simulation:");
    
    struct ConversationSession {
        client: OpenAIClient,
        history: Vec<String>,
        max_history: usize,
    }
    
    impl ConversationSession {
        fn new(client: OpenAIClient) -> Self {
            Self {
                client,
                history: Vec::new(),
                max_history: 5,
            }
        }
        
        async fn send_message(&mut self, message: &str) -> Result<String, Box<dyn std::error::Error> {
            self.history.push(format!("User: {}", message));
            
            // Keep only recent messages
            if self.history.len() > self.max_history {
                self.history = self.history[self.history.len() - self.max_history..].to_vec();
            }
            
            let context = self.history.join("\n");
            let response = self.client
                .create_response_builder(Model::Gpt4_1Nano, message)
                .instructions(&format!("Conversation context:\n{}\n\nRespond naturally.", context))
                .max_tokens(150)
                .send()
                .await?;
            
            let reply = response.get_text_output().unwrap_or_default();
            self.history.push(format!("Assistant: {}", reply));
            
            Ok(reply)
        }
    }
    
    let mut session = ConversationSession::new(client);
    
    println!("Session started...");
    let reply1 = session.send_message("What's your favorite Rust feature?").await?;
    println!("Assistant: {}", reply1);
    
    let reply2 = session.send_message("Why do you prefer that over others?").await?;
    println!("Assistant: {}", reply2);
    
    Ok(())
}

// Run this example with:
// cargo run --example 03_conversation_state