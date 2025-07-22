//! # Web Search Integration
//!
//! This example demonstrates how to use web search capabilities
//! with the Responses API to get real-time information.

use openai_responses::{OpenAIClient, Model, CreateResponseRequest, Tool, ToolFunction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::from_env()?;
    
    println!("=== Web Search Integration ===\n");
    
    // Example 1: Current events search
    println!("1. Current events search:");
    let web_search_tool = Tool {
        tool_type: "web_search_preview".to_string(),
        function: Some(ToolFunction {
            name: "web_search".to_string(),
            description: Some("Search the web for current information".to_string()),
            parameters: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {"type": "string", "description": "Search query"},
                    "context_size": {"type": "string", "enum": ["low", "medium", "high"]}
                },
                "required": ["query"]
            })),
        }),
    };
    
    let request = CreateResponseRequest::new(Model::Gpt4_1Nano, "What are the latest developments in Rust programming language in 2024?")
        .with_tools(vec![web_search_tool])
        .with_instructions("Use web search to find the most recent information about Rust developments.");
    
    let response = client.create_response(&request).await?;
    println!("Search results: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 2: Technology news aggregation
    println!("2. Technology news aggregation:");
    let tech_search_request = CreateResponseRequest::new(
        Model::Gpt4_1Nano,
        "Find recent AI breakthroughs and summarize the key points"
    )
    .with_tools(vec![
        Tool {
            tool_type: "web_search_preview".to_string(),
            function: Some(ToolFunction {
                name: "search_tech_news".to_string(),
                description: Some("Search for technology news and breakthroughs".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "topic": {"type": "string"},
                        "timeframe": {"type": "string", "enum": ["recent", "week", "month"]}
                    }
                })),
            }),
        }
    ])
    .with_instructions("Search for recent AI breakthroughs and provide a concise summary.");
    
    let response = client.create_response(&tech_search_request).await?;
    println!("AI news summary: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 3: Market research
    println!("3. Market research with web search:");
    let market_research = CreateResponseRequest::new(
        Model::Gpt4_1Nano,
        "What is the current market share of programming languages in 2024?"
    )
    .with_tools(vec![
        Tool {
            tool_type: "web_search_preview".to_string(),
            function: Some(ToolFunction {
                name: "search_market_data".to_string(),
                description: Some("Search for market research data".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": {"type": "string"},
                        "source": {"type": "string", "enum": ["stackoverflow", "github", "industry"]}
                    }
                })),
            }),
        }
    ])
    .with_instructions("Find current programming language market share statistics and trends.");
    
    let response = client.create_response(&market_research).await?;
    println!("Market research: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 4: Real-time data analysis
    println!("4. Real-time data analysis:");
    let realtime_analysis = CreateResponseRequest::new(
        Model::Gpt4_1Nano,
        "Analyze the current state of the Rust ecosystem"
    )
    .with_tools(vec![
        Tool {
            tool_type: "web_search_preview".to_string(),
            function: Some(ToolFunction {
                name: "analyze_ecosystem".to_string(),
                description: Some("Analyze ecosystem health and trends".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "ecosystem": {"type": "string"},
                        "metrics": {"type": "array", "items": {"type": "string"}}
                    }
                })),
            }),
        }
    ])
    .with_instructions("Search for recent data about Rust ecosystem growth, adoption, and community metrics.");
    
    let response = client.create_response(&realtime_analysis).await?;
    println!("Ecosystem analysis: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 5: Comparative analysis
    println!("5. Comparative web search analysis:");
    let comparative_search = CreateResponseRequest::new(
        Model::Gpt4_1Nano,
        "Compare Rust vs Go performance benchmarks in 2024"
    )
    .with_tools(vec![
        Tool {
            tool_type: "web_search_preview".to_string(),
            function: Some(ToolFunction {
                name: "compare_technologies".to_string(),
                description: Some("Compare technology benchmarks and performance".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "tech1": {"type": "string"},
                        "tech2": {"type": "string"},
                        "metric": {"type": "string"}
                    }
                })),
            }),
        }
    ])
    .with_instructions("Search for recent performance benchmarks comparing Rust and Go, focusing on 2024 data.");
    
    let response = client.create_response(&comparative_search).await?;
    println!("Technology comparison: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 6: News summarization pipeline
    println!("6. News summarization pipeline:");
    let news_pipeline = CreateResponseRequest::new(
        Model::Gpt4_1Nano,
        "Summarize the top 5 tech news stories from this week"
    )
    .with_tools(vec![
        Tool {
            tool_type: "web_search_preview".to_string(),
            function: Some(ToolFunction {
                name: "summarize_news".to_string(),
                description: Some("Summarize news articles".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "category": {"type": "string", "enum": ["tech", "science", "business"]},
                        "count": {"type": "integer", "minimum": 1, "maximum": 10},
                        "timeframe": {"type": "string"}
                    }
                })),
            }),
        }
    ])
    .with_instructions("Search for recent tech news and create a concise summary of the top 5 stories.");
    
    let response = client.create_response(&news_pipeline).await?;
    println!("Weekly tech summary: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 7: Research assistant workflow
    println!("7. Research assistant workflow:");
    let research_workflow = CreateResponseRequest::new(
        Model::Gpt4_1Nano,
        "Create a research report on quantum computing applications in 2024"
    )
    .with_tools(vec![
        Tool {
            tool_type: "web_search_preview".to_string(),
            function: Some(ToolFunction {
                name: "research_topic".to_string(),
                description: Some("Research a specific topic comprehensively".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "topic": {"type": "string"},
                        "depth": {"type": "string", "enum": ["overview", "detailed", "comprehensive"]},
                        "sources": {"type": "array", "items": {"type": "string"}}
                    }
                })),
            }),
        }
    ])
    .with_instructions("Conduct comprehensive research on quantum computing applications and provide a structured report.");
    
    let response = client.create_response(&research_workflow).await?;
    println!("Research report: {}\n", response.get_text_output().unwrap_or_default());
    
    Ok(())
}

// Run this example with:
// cargo run --example 06_web_search