//! # Multimodal Content Processing
//!
//! This example demonstrates working with multimodal content including
//! text, images, and mixed content types using the updated API.

use openai_responses::{
    OpenAIClient, Model, CreateResponseRequest, 
    Content, ContentType, Message, Output, Modality
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::from_env()?;
    
    println!("=== Multimodal Content Processing ===\n");
    
    // Check model capabilities
    let vision_model = Model::Gpt4_1;
    let standard_model = Model::Gpt4_1Nano;
    
    let model = if vision_model.supports_vision() {
        println!("👁️  Using vision-capable model: {}", vision_model.as_str());
        vision_model
    } else {
        println!("⚠️  Vision model not available, using standard model: {}", standard_model.as_str());
        standard_model
    };
    
    // Example 1: Text-only content
    println!("1. Text-only content processing:");
    let text_content = Content {
        content_type: ContentType::Text,
        text: Some("Analyze the benefits of Rust programming language".to_string()),
        image_url: None,
    };
    
    println!("Content type: {:?}", text_content.content_type);
    if let Some(text) = &text_content.text {
        println!("Text content: {}", text);
    }
    
    let response = client
        .create_simple_response(model, "Explain Rust's memory safety features")
        .await?;
    
    println!("Response: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 2: Image content (if vision is supported)
    if model.supports_vision() {
        println!("2. Image content processing:");
        
        let mut request = CreateResponseRequest::new(
            model,
            "Analyze this image and describe what you see"
        )
        .with_modalities(vec![Modality::Text, Modality::Image])
        .with_instructions("Provide a detailed description of the image content");
        
        // Note: In a real application, you would provide an actual image URL
        let image_content = Content {
            content_type: ContentType::ImageUrl,
            text: None,
            image_url: Some("https://example.com/rust-logo.png".to_string()),
        };
        
        println!("Image content type: {:?}", image_content.content_type);
        if let Some(url) = &image_content.image_url {
            println!("Image URL: {}", url);
        }
        
        // For demonstration, we'll just show the structure
        println!("✅ Image content structure created successfully");
    } else {
        println!("2. Image content processing: ⚠️  Skipped (model doesn't support vision)");
    }
    
    // Example 3: Mixed content types
    println!("\n3. Mixed content types:");
    let mixed_contents = vec![
        Content {
            content_type: ContentType::Text,
            text: Some("Here's some explanatory text".to_string()),
            image_url: None,
        },
        Content {
            content_type: ContentType::ImageUrl,
            text: None,
            image_url: Some("https://example.com/diagram.png".to_string()),
        },
        Content {
            content_type: ContentType::Text,
            text: Some("And here's more text after the image".to_string()),
            image_url: None,
        },
    ];
    
    println!("Mixed content items: {}", mixed_contents.len());
    for (i, content) in mixed_contents.iter().enumerate() {
        match content.content_type {
            ContentType::Text => {
                if let Some(text) = &content.text {
                    println!("  {}. Text: {}", i + 1, text);
                }
            },
            ContentType::ImageUrl => {
                if let Some(url) = &content.image_url {
                    println!("  {}. Image: {}", i + 1, url);
                }
            },
            ContentType::Refusal => {
                println!("  {}. Refusal content", i + 1);
            },
        }
    }
    
    // Example 4: Message structure
    println!("\n4. Message structure:");
    let user_message = Message {
        role: "user".to_string(),
        content: vec![
            Content {
                content_type: ContentType::Text,
                text: Some("What do you think about this code?".to_string()),
                image_url: None,
            }
        ],
    };
    
    let assistant_message = Message {
        role: "assistant".to_string(),
        content: vec![
            Content {
                content_type: ContentType::Text,
                text: Some("This code looks well-structured and follows Rust best practices.".to_string()),
                image_url: None,
            }
        ],
    };
    
    println!("User message role: {}", user_message.role);
    println!("User message content items: {}", user_message.content.len());
    println!("Assistant message role: {}", assistant_message.role);
    println!("Assistant message content items: {}", assistant_message.content.len());
    
    // Example 5: Output processing
    println!("\n5. Output processing:");
    let sample_output = Output {
        output_type: "text".to_string(),
        content: vec![
            Content {
                content_type: ContentType::Text,
                text: Some("This is the generated response text.".to_string()),
                image_url: None,
            }
        ],
    };
    
    println!("Output type: {}", sample_output.output_type);
    println!("Output content items: {}", sample_output.content.len());
    
    // Process output content
    for (i, content) in sample_output.content.iter().enumerate() {
        match content.content_type {
            ContentType::Text => {
                if let Some(text) = &content.text {
                    println!("  Output {}: {}", i + 1, text);
                }
            },
            ContentType::ImageUrl => {
                if let Some(url) = &content.image_url {
                    println!("  Output {}: Image at {}", i + 1, url);
                }
            },
            ContentType::Refusal => {
                println!("  Output {}: Model refused to generate content", i + 1);
            },
        }
    }
    
    // Example 6: Content type checking and validation
    println!("\n6. Content type checking and validation:");
    
    fn validate_content(content: &Content) -> Result<(), String> {
        match content.content_type {
            ContentType::Text => {
                if content.text.is_none() {
                    return Err("Text content must have text field".to_string());
                }
                if content.image_url.is_some() {
                    return Err("Text content should not have image_url field".to_string());
                }
            },
            ContentType::ImageUrl => {
                if content.image_url.is_none() {
                    return Err("Image content must have image_url field".to_string());
                }
                if content.text.is_some() {
                    return Err("Image content should not have text field".to_string());
                }
            },
            ContentType::Refusal => {
                // Refusal content typically has neither text nor image_url
                // but this depends on the specific API implementation
            },
        }
        Ok(())
    }
    
    let test_contents = vec![
        Content {
            content_type: ContentType::Text,
            text: Some("Valid text content".to_string()),
            image_url: None,
        },
        Content {
            content_type: ContentType::ImageUrl,
            text: None,
            image_url: Some("https://example.com/image.jpg".to_string()),
        },
        Content {
            content_type: ContentType::Text,
            text: None, // Invalid: missing text
            image_url: None,
        },
    ];
    
    for (i, content) in test_contents.iter().enumerate() {
        match validate_content(content) {
            Ok(()) => println!("✅ Content {} is valid", i + 1),
            Err(e) => println!("❌ Content {} is invalid: {}", i + 1, e),
        }
    }
    
    // Example 7: Modality configuration
    println!("\n7. Modality configuration:");
    
    let text_only_modalities = vec![Modality::Text];
    let multimodal_modalities = vec![Modality::Text, Modality::Image];
    let audio_modalities = vec![Modality::Text, Modality::Audio];
    
    println!("Text-only modalities: {:?}", text_only_modalities);
    println!("Multimodal modalities: {:?}", multimodal_modalities);
    println!("Audio modalities: {:?}", audio_modalities);
    
    // Check which modalities are supported by the model
    if model.supports_vision() {
        println!("✅ Model supports vision (image modality)");
    } else {
        println!("❌ Model does not support vision");
    }
    
    // Note: Audio support would need to be checked similarly
    // if model.supports_audio() { ... }
    
    println!("\n💡 Multimodal best practices:");
    println!("   - Always check model capabilities before using modalities");
    println!("   - Validate content structure before sending requests");
    println!("   - Use appropriate content types for each piece of content");
    println!("   - Handle refusal content gracefully");
    println!("   - Consider token costs for different content types");
    
    Ok(())
}

// Run this example with:
// cargo run --example multimodal_content
