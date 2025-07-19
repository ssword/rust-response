//! # Function Calling with Tools
//!
//! This example demonstrates how to use function calling capabilities
//! with the Responses API to enable external tool usage.

use openai_responses::{OpenAIClient, Model, CreateResponseRequest, Tool, ToolFunction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::from_env()?;
    
    println!("=== Function Calling with Tools ===\n");
    
    // Example 1: Weather function
    println!("1. Weather lookup function:");
    let weather_tool = Tool {
        tool_type: "function".to_string(),
        function: Some(ToolFunction {
            name: "get_weather".to_string(),
            description: Some("Get current weather for a location".to_string()),
            parameters: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "City and country"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"],
                        "description": "Temperature unit"
                    }
                },
                "required": ["location"]
            })),
        }),
    };
    
    // Check if the model supports function calling
    let model = Model::Gpt4_1Nano;
    if !model.supports_function_calling() {
        println!("⚠️  Model {} doesn't support function calling. Using a compatible model instead.", model.as_str());
    }

    let request = CreateResponseRequest::new(model, "What's the weather like in Tokyo?")
        .with_tools(vec![weather_tool])
        .with_instructions("Use the provided weather function to answer the question.");
    
    let response = client.create_response(&request).await?;
    println!("Response: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 2: Calculator functions
    println!("2. Calculator functions:");
    let calc_tools = vec![
        Tool {
            tool_type: "function".to_string(),
            function: Some(ToolFunction {
                name: "add".to_string(),
                description: Some("Add two numbers".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "a": {"type": "number"},
                        "b": {"type": "number"}
                    },
                    "required": ["a", "b"]
                })),
            }),
        },
        Tool {
            tool_type: "function".to_string(),
            function: Some(ToolFunction {
                name: "multiply".to_string(),
                description: Some("Multiply two numbers".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "a": {"type": "number"},
                        "b": {"type": "number"}
                    },
                    "required": ["a", "b"]
                })),
            }),
        },
    ];
    
    let request = CreateResponseRequest::new(Model::Gpt4_1Nano, "Calculate 15 + 27 and then multiply by 3")
        .with_tools(calc_tools)
        .with_instructions("Use the provided calculator functions to solve the problem step by step.");
    
    let response = client.create_response(&request).await?;
    println!("Calculation result: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 3: Database query simulation
    println!("3. Database query simulation:");
    let db_tools = vec![
        Tool {
            tool_type: "function".to_string(),
            function: Some(ToolFunction {
                name: "query_users".to_string(),
                description: Some("Query user database".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "field": {"type": "string", "enum": ["name", "email", "age"]},
                        "value": {"type": "string"}
                    },
                    "required": ["field", "value"]
                })),
            }),
        },
        Tool {
            tool_type: "function".to_string(),
            function: Some(ToolFunction {
                name: "get_user_stats".to_string(),
                description: Some("Get user statistics".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "user_id": {"type": "integer"}
                    },
                    "required": ["user_id"]
                })),
            }),
        },
    ];
    
    let request = CreateResponseRequest::new(Model::Gpt4_1Nano, "Find users named Alice and get their statistics")
        .with_tools(db_tools)
        .with_instructions("Use the database functions to find users and get their stats.");
    
    let response = client.create_response(&request).await?;
    println!("Database query result: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 4: Function calling with validation
    println!("4. Function validation and error handling:");
    
    #[derive(Debug)]
    struct FunctionCall {
        name: String,
        arguments: serde_json::Value,
    }
    
    impl FunctionCall {
        fn validate(&self) -> Result<(), String> {
            match self.name.as_str() {
                "get_weather" => {
                    if let Some(location) = self.arguments["location"].as_str() {
                        if location.is_empty() {
                            return Err("Location cannot be empty".to_string());
                        }
                        Ok(())
                    } else {
                        Err("Missing location parameter".to_string())
                    }
                },
                "add" | "multiply" => {
                    let a = self.arguments["a"].as_f64().ok_or("Missing parameter 'a'")?;
                    let b = self.arguments["b"].as_f64().ok_or("Missing parameter 'b'")?;
                    Ok(())
                },
                _ => Err(format!("Unknown function: {}", self.name)),
            }
        }
    }
    
    // Example 5: Complex multi-function workflow
    println!("5. Multi-function workflow:");
    let workflow_tools = vec![
        Tool {
            tool_type: "function".to_string(),
            function: Some(ToolFunction {
                name: "get_stock_price".to_string(),
                description: Some("Get current stock price".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "symbol": {"type": "string", "description": "Stock symbol"}
                    },
                    "required": ["symbol"]
                })),
            }),
        },
        Tool {
            tool_type: "function".to_string(),
            function: Some(ToolFunction {
                name: "convert_currency".to_string(),
                description: Some("Convert between currencies".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "amount": {"type": "number"},
                        "from": {"type": "string"},
                        "to": {"type": "string"}
                    },
                    "required": ["amount", "from", "to"]
                })),
            }),
        },
    ];
    
    let request = CreateResponseRequest::new(Model::Gpt4_1Nano, "Get Apple's stock price and convert it to EUR for 100 shares")
        .with_tools(workflow_tools)
        .with_instructions("Use multiple functions to solve this investment query step by step.");
    
    let response = client.create_response(&request).await?;
    println!("Investment analysis: {}\n", response.get_text_output().unwrap_or_default());
    
    Ok(())
}

// Run this example with:
// cargo run --example 04_function_calling