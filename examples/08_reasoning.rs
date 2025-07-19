//! # Complex Reasoning and Multi-step Problems
//!
//! This example demonstrates advanced reasoning capabilities including
//! multi-step problem solving, logical deduction, and complex analysis.

use openai_responses::{OpenAIClient, CreateResponseRequest, Tool, ToolFunction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::from_env()?;
    
    println!("=== Complex Reasoning and Multi-step Problems ===\n");
    
    // Example 1: Mathematical problem solving
    println!("1. Mathematical problem solving:");
    let response = client
        .create_response_builder("gpt-4.1-nano", "Solve this step by step: A train leaves Chicago at 2:00 PM traveling at 60 mph. Another train leaves New York at 3:00 PM traveling at 80 mph. The distance between Chicago and New York is 790 miles. When do they meet?")
        .instructions("Break this down into clear, logical steps and show your reasoning process.")
        .max_tokens(300)
        .send()
        .await?;
    
    println!("Solution:\n{}\n", response.get_text_output().unwrap_or_default());
    
    // Example 2: Logical deduction
    println!("2. Logical deduction puzzle:");
    let response = client
        .create_response_builder("gpt-4.1-nano", "Three people - Alice, Bob, and Carol - have different professions: doctor, engineer, and teacher. Alice is not the doctor. Bob is taller than the teacher. The engineer is shorter than Carol. Determine each person's profession.")
        .instructions("Use systematic logical deduction to solve this puzzle. Show your reasoning step by step.")
        .max_tokens(200)
        .send()
        .await?;
    
    println!("Deduction:\n{}\n", response.get_text_output().unwrap_or_default());
    
    // Example 3: Business strategy analysis
    println!("3. Business strategy analysis:");
    let response = client
        .create_response_builder("gpt-4.1-nano", "A tech startup has $100K in funding, 5 team members, and 6 months runway. They need to decide between: 1) Building an MVP and seeking Series A, 2) Focusing on enterprise sales, or 3) Pursuing a freemium model. Analyze the pros and cons of each approach.")
        .instructions("Provide comprehensive analysis including risks, benefits, and recommendations for each option.")
        .max_tokens(400)
        .send()
        .await?;
    
    println!("Strategy analysis:\n{}\n", response.get_text_output().unwrap_or_default());
    
    // Example 4: Scientific reasoning
    println!("4. Scientific reasoning:");
    let response = client
        .create_response_builder("gpt-4.1-nano", "Explain why ice floats on water using molecular structure and density principles. Then explain why this property is crucial for aquatic life.")
        .instructions("Use scientific reasoning with clear explanations of molecular structure and its biological implications.")
        .max_tokens(250)
        .send()
        .await?;
    
    println!("Scientific explanation:\n{}\n", response.get_text_output().unwrap_or_default());
    
    // Example 5: Code optimization reasoning
    println!("5. Code optimization reasoning:");
    let optimization_request = CreateResponseRequest::new(
        "gpt-4.1-nano",
        "Given this function that calculates the nth Fibonacci number using recursion, optimize it for better performance while maintaining readability."
    )
    .with_tools(vec![
        Tool {
            tool_type: "function".to_string(),
            function: Some(ToolFunction {
                name: "analyze_algorithm".to_string(),
                description: Some("Analyze algorithm complexity and suggest optimizations".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "algorithm": {"type": "string"},
                        "input_size": {"type": "integer"},
                        "constraints": {"type": "array", "items": {"type": "string"}}
                    }
                })),
            }),
        }
    ])
    .with_instructions("Analyze the Fibonacci algorithm and provide optimization reasoning with trade-offs.");
    
    let response = client.create_response(&optimization_request).await?;
    println!("Optimization analysis:\n{}\n", response.get_text_output().unwrap_or_default());
    
    // Example 6: Ethical reasoning
    println!("6. Ethical reasoning:");
    let response = client
        .create_response_builder("gpt-4.1-nano", "A self-driving car faces an unavoidable accident scenario. It must choose between hitting one elderly person or swerving and potentially hitting five younger people. Analyze the ethical considerations from multiple perspectives.")
        .instructions("Provide comprehensive ethical analysis including utilitarian, deontological, and virtue ethics perspectives.")
        .max_tokens(350)
        .send()
        .await?;
    
    println!("Ethical analysis:\n{}\n", response.get_text_output().unwrap_or_default());
    
    // Example 7: Financial planning reasoning
    println!("7. Financial planning reasoning:");
    let financial_reasoning = CreateResponseRequest::new(
        "gpt-4.1-nano",
        "A 30-year-old software engineer earning $120K annually wants to retire by 50. They have $50K saved, no debt, and can save 30% of income. Calculate how much they need to invest monthly to reach $2M by retirement, considering 7% annual returns."
    )
    .with_tools(vec![
        Tool {
            tool_type: "function".to_string(),
            function: Some(ToolFunction {
                name: "calculate_retirement".to_string(),
                description: Some("Calculate retirement planning parameters".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "current_age": {"type": "integer"},
                        "retirement_age": {"type": "integer"},
                        "current_savings": {"type": "number"},
                        "annual_income": {"type": "number"},
                        "savings_rate": {"type": "number"},
                        "target_amount": {"type": "number"},
                        "expected_return": {"type": "number"}
                    }
                })),
            }),
        }
    ])
    .with_instructions("Calculate retirement planning parameters and provide investment strategy reasoning.");
    
    let response = client.create_response(&financial_reasoning).await?;
    println!("Financial planning:\n{}\n", response.get_text_output().unwrap_or_default());
    
    // Example 8: Multi-step problem with verification
    println!("8. Multi-step problem with verification:");
    let multi_step_tools = vec![
        Tool {
            tool_type: "function".to_string(),
            function: Some(ToolFunction {
                name: "solve_step".to_string(),
                description: Some("Solve individual problem steps".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "problem": {"type": "string"},
                        "step": {"type": "integer"},
                        "context": {"type": "string"}
                    }
                })),
            }),
        },
        Tool {
            tool_type: "function".to_string(),
            function: Some(ToolFunction {
                name: "verify_solution".to_string(),
                description: Some("Verify solution correctness".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "solution": {"type": "string"},
                        "expected_result": {"type": "string"}
                    }
                })),
            }),
        }
    ];
    
    let request = CreateResponseRequest::new(
        "gpt-4.1-nano",
        "Design a load balancing algorithm for a distributed system with 5 servers, varying loads, and 1000 concurrent users. Consider failover scenarios and performance optimization."
    )
    .with_tools(multi_step_tools)
    .with_instructions("Break this complex problem into steps: 1) Requirements analysis, 2) Algorithm design, 3) Implementation considerations, 4) Testing strategy, 5) Optimization.");
    
    let response = client.create_response(&request).await?;
    println!("System design:\n{}\n", response.get_text_output().unwrap_or_default());
    
    // Example 9: Chain of thought reasoning
    println!("9. Chain of thought reasoning:");
    let response = client
        .create_response_builder("gpt-4.1-nano", "Consider a startup that has developed a revolutionary battery technology. Walk through their go-to-market strategy from prototype to mass production, considering regulatory approval, manufacturing scaling, and market competition.")
        .instructions("Use systematic chain-of-thought reasoning to analyze each phase step by step, considering dependencies and critical decisions.")
        .max_tokens(500)
        .send()
        .await?;
    
    println!("Strategic reasoning:\n{}\n", response.get_text_output().unwrap_or_default());
    
    // Example 10: Comparative reasoning
    println!("10. Comparative reasoning:");
    let response = client
        .create_response_builder("gpt-4.1-nano", "Compare the economic and social impacts of remote work adoption across three scenarios: 1) 100% remote, 2) Hybrid model, 3) Return to office. Consider productivity, employee satisfaction, urban planning, and real estate markets.")
        .instructions("Provide systematic comparative analysis with evidence-based reasoning for each scenario.")
        .max_tokens(450)
        .send()
        .await?;
    
    println!("Comparative analysis:\n{}\n", response.get_text_output().unwrap_or_default());
    
    Ok(())
}

// Run this example with:
// cargo run --example 08_reasoning