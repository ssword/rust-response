//! # File Search and Analysis
//!
//! This example demonstrates how to use file search capabilities
//! with the Responses API to analyze and search through documents.

use openai_responses::{OpenAIClient, CreateResponseRequest, Tool, ToolFunction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::from_env()?;
    
    println!("=== File Search and Analysis ===\n");
    
    // Example 1: Document analysis
    println!("1. Document content analysis:");
    let file_search_tool = Tool {
        tool_type: "file_search".to_string(),
        function: Some(ToolFunction {
            name: "analyze_document".to_string(),
            description: Some("Analyze document content and extract key information".to_string()),
            parameters: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "file_path": {"type": "string", "description": "Path to the document"},
                    "analysis_type": {"type": "string", "enum": ["summary", "keywords", "sentiment"]}
                },
                "required": ["file_path", "analysis_type"]
            })),
        }),
    };
    
    let request = CreateResponseRequest::new("gpt-4.1-nano", "Analyze a Rust source code file for code patterns and best practices")
        .with_tools(vec![file_search_tool])
        .with_instructions("Use file search to analyze Rust code files and provide insights.");
    
    let response = client.create_response(&request).await?;
    println!("Analysis insights: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 2: Code review simulation
    println!("2. Code review simulation:");
    let code_review_tools = vec![
        Tool {
            tool_type: "file_search".to_string(),
            function: Some(ToolFunction {
                name: "review_code".to_string(),
                description: Some("Review code for quality and issues".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "code": {"type": "string"},
                        "language": {"type": "string"},
                        "focus_areas": {"type": "array", "items": {"type": "string"}}
                    }
                })),
            }),
        }
    ];
    
    let sample_code = r#"
    fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }
    "#;
    
    let request = CreateResponseRequest::new(
        "gpt-4.1-nano", 
        &format!("Review this Rust code:\n{}", sample_code)
    )
    .with_tools(code_review_tools)
    .with_instructions("Analyze the provided Rust code for performance, safety, and best practices.");
    
    let response = client.create_response(&request).await?;
    println!("Code review: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 3: Documentation extraction
    println!("3. Documentation extraction:");
    let doc_tools = vec![
        Tool {
            tool_type: "file_search".to_string(),
            function: Some(ToolFunction {
                name: "extract_docs".to_string(),
                description: Some("Extract documentation from source files".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "file_content": {"type": "string"},
                        "extract_type": {"type": "string", "enum": ["comments", "api_docs", "examples"]}
                    }
                })),
            }),
        }
    ];
    
    let documented_code = r#"
    /// Calculates the area of a circle given its radius
    /// 
    /// # Arguments
    /// * `radius` - The radius of the circle
    /// 
    /// # Returns
    /// The area of the circle as f64
    /// 
    /// # Example
    /// ```
    /// let area = circle_area(5.0);
    /// assert_eq!(area, 78.53981633974483);
    /// ```
    pub fn circle_area(radius: f64) -> f64 {
        std::f64::consts::PI * radius * radius
    }
    "#;
    
    let request = CreateResponseRequest::new(
        "gpt-4.1-nano",
        &format!("Extract documentation from:\n{}", documented_code)
    )
    .with_tools(doc_tools)
    .with_instructions("Extract and organize the documentation from the provided Rust code.");
    
    let response = client.create_response(&request).await?;
    println!("Documentation: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 4: File content search
    println!("4. File content search:");
    let search_tools = vec![
        Tool {
            tool_type: "file_search".to_string(),
            function: Some(ToolFunction {
                name: "search_files".to_string(),
                description: Some("Search within file contents".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": {"type": "string"},
                        "file_types": {"type": "array", "items": {"type": "string"}},
                        "directory": {"type": "string"}
                    }
                })),
            }),
        }
    ];
    
    let request = CreateResponseRequest::new(
        "gpt-4.1-nano",
        "Search for all TODO comments in Rust source files"
    )
    .with_tools(search_tools)
    .with_instructions("Simulate searching through Rust project files for TODO comments.");
    
    let response = client.create_response(&request).await?;
    println!("Search results: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 5: Log file analysis
    println!("5. Log file analysis:");
    let log_analysis_tools = vec![
        Tool {
            tool_type: "file_search".to_string(),
            function: Some(ToolFunction {
                name: "analyze_logs".to_string(),
                description: Some("Analyze log files for patterns and issues".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "log_content": {"type": "string"},
                        "level": {"type": "string", "enum": ["error", "warning", "info"]},
                        "time_range": {"type": "string"}
                    }
                })),
            }),
        }
    ];
    
    let sample_logs = r#"
    [2024-01-15 10:30:45] ERROR: Database connection failed
    [2024-01-15 10:31:12] WARNING: High memory usage detected
    [2024-01-15 10:32:01] INFO: Application started successfully
    [2024-01-15 10:33:22] ERROR: Failed to process user request
    "#;
    
    let request = CreateResponseRequest::new(
        "gpt-4.1-nano",
        &format!("Analyze these logs:\n{}", sample_logs)
    )
    .with_tools(log_analysis_tools)
    .with_instructions("Analyze the provided log data for errors, warnings, and patterns.");
    
    let response = client.create_response(&request).await?;
    println!("Log analysis: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 6: Configuration analysis
    println!("6. Configuration file analysis:");
    let config_tools = vec![
        Tool {
            tool_type: "file_search".to_string(),
            function: Some(ToolFunction {
                name: "parse_config".to_string(),
                description: Some("Parse and validate configuration files".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "config_text": {"type": "string"},
                        "format": {"type": "string", "enum": ["toml", "yaml", "json"]}
                    }
                })),
            }),
        }
    ];
    
    let sample_config = r#"
    [server]
    host = "localhost"
    port = 8080
    
    [database]
    url = "postgres://user:pass@localhost/db"
    pool_size = 10
    
    [logging]
    level = "info"
    file = "app.log"
    "#;
    
    let request = CreateResponseRequest::new(
        "gpt-4.1-nano",
        &format!("Parse this TOML config:\n{}", sample_config)
    )
    .with_tools(config_tools)
    .with_instructions("Parse the configuration file and provide validation insights.");
    
    let response = client.create_response(&request).await?;
    println!("Config analysis: {}\n", response.get_text_output().unwrap_or_default());
    
    // Example 7: Security audit simulation
    println!("7. Security audit simulation:");
    let security_tools = vec![
        Tool {
            tool_type: "file_search".to_string(),
            function: Some(ToolFunction {
                name: "security_scan".to_string(),
                description: Some("Scan code for security vulnerabilities".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "code": {"type": "string"},
                        "language": {"type": "string"},
                        "scan_type": {"type": "string", "enum": ["sql_injection", "xss", "auth", "crypto"]}
                    }
                })),
            }),
        }
    ];
    
    let vulnerable_code = r#"
    use sqlx::PgPool;
    
    async fn get_user(pool: &PgPool, username: &str) -> Result<User, sqlx::Error> {
        let query = format!("SELECT * FROM users WHERE username = '{}'", username);
        sqlx::query_as::<_, User>(&query).fetch_one(pool).await
    }
    "#;
    
    let request = CreateResponseRequest::new(
        "gpt-4.1-nano",
        &format!("Security scan this code:\n{}", vulnerable_code)
    )
    .with_tools(security_tools)
    .with_instructions("Scan the provided Rust code for security vulnerabilities and provide recommendations.");
    
    let response = client.create_response(&request).await?;
    println!("Security scan: {}\n", response.get_text_output().unwrap_or_default());
    
    Ok(())
}

// Run this example with:
// cargo run --example 07_file_search