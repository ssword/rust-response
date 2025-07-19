#[cfg(test)]
mod tests {
    use openai_responses::{OpenAIConfig, OpenAIClient};
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, header, body_json};

    #[tokio::test]
    async fn test_full_response_lifecycle() {
        let mock_server = MockServer::start().await;
        
        // Mock create response
        let create_body = r#"{
            "id": "resp_123",
            "object": "response",
            "status": "completed",
            "output": [{"type": "message", "content": [{"type": "text", "text": "Hello from mock"}]}],
            "usage": {"total_tokens": 5}
        }"#;

        Mock::given(method("POST"))
            .and(path("/responses"))
            .and(header("Authorization", "Bearer test-key"))
            .and(body_json(serde_json::json!({
                "model": "gpt-4.1-nano",
                "input": "Hello"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_string(create_body))
            .mount(&mock_server)
            .await;

        // Mock get response
        let get_body = r#"{
            "id": "resp_123",
            "object": "response",
            "status": "completed",
            "output": [{"type": "message", "content": [{"type": "text", "text": "Hello from mock"}]}],
            "usage": {"total_tokens": 5}
        }"#;

        Mock::given(method("GET"))
            .and(path("/responses/resp_123"))
            .respond_with(ResponseTemplate::new(200).set_body_string(get_body))
            .mount(&mock_server)
            .await;

        // Mock delete response
        let delete_body = r#"{
            "id": "resp_123",
            "object": "response.deleted",
            "deleted": true
        }"#;

        Mock::given(method("DELETE"))
            .and(path("/responses/resp_123"))
            .respond_with(ResponseTemplate::new(200).set_body_string(delete_body))
            .mount(&mock_server)
            .await;

        let config = OpenAIConfig::new("test-key")
            .with_base_url(mock_server.uri());
        let client = OpenAIClient::new(config).unwrap();

        // Test full lifecycle
        let response = client.create_simple_response("gpt-4.1-nano", "Hello").await.unwrap();
        assert_eq!(response.id, "resp_123");
        
        let retrieved = client.get_response("resp_123").await.unwrap();
        assert_eq!(retrieved.id, "resp_123");
        
        let deleted = client.delete_response("resp_123").await.unwrap();
        assert!(deleted.deleted);
    }

    #[tokio::test]
    async fn test_builder_pattern() {
        let mock_server = MockServer::start().await;
        
        let response_body = r#"{
            "id": "resp_456",
            "object": "response",
            "status": "completed",
            "output": [{"type": "message", "content": [{"type": "text", "text": "Built response"}]}],
            "usage": {"total_tokens": 10}
        }"#;

        Mock::given(method("POST"))
            .and(path("/responses"))
            .and(body_json(serde_json::json!({
                "model": "gpt-4.1-nano",
                "input": "Test input",
                "temperature": 0.8,
                "max_tokens": 100
            })))
            .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
            .mount(&mock_server)
            .await;

        let config = OpenAIConfig::new("test-key")
            .with_base_url(mock_server.uri());
        let client = OpenAIClient::new(config).unwrap();

        let response = client
            .create_response_builder("gpt-4.1-nano", "Test input")
            .temperature(0.8)
            .max_tokens(100)
            .send()
            .await
            .unwrap();

        assert_eq!(response.id, "resp_456");
        assert!(response.is_completed());
    }

    #[tokio::test]
    async fn test_error_handling() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("POST"))
            .and(path("/responses"))
            .respond_with(ResponseTemplate::new(401).set_body_string(r#"{"error": {"message": "Invalid API key", "type": "invalid_request_error"}}"#))
            .mount(&mock_server)
            .await;

        let config = OpenAIConfig::new("invalid-key")
            .with_base_url(mock_server.uri());
        let client = OpenAIClient::new(config).unwrap();

        let result = client.create_simple_response("gpt-4.1-nano", "Hello").await;
        assert!(result.is_err());
        
        match result {
            Err(openai_responses::OpenAIError::Authentication(msg)) => {
                assert!(msg.contains("Invalid API key"));
            }
            _ => panic!("Expected authentication error"),
        }
    }
}