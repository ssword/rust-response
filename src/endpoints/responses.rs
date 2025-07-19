use crate::client::OpenAIClient;
use crate::error::Result;
use crate::types::{CreateResponseRequest, Response, DeletedResponse, ResponseList};

impl OpenAIClient {
    /// Create a new response
    pub async fn create_response(&self, request: &CreateResponseRequest) -> Result<Response> {
        let response = self
            .execute_with_retry(self.post("responses").json(request))
            .await?;
        
        response.json().await.map_err(Into::into)
    }

    /// Create a response with just model and input
    pub async fn create_simple_response(&self, model: Model, input: &str) -> Result<Response> {
        let request = CreateResponseRequest::new(model, input);
        self.create_response(&request).await
    }

    /// Create a response builder for fluent API
    pub fn create_response_builder(
        &self, model: Model, input: impl Into<String>
    ) -> ResponseBuilder {
        ResponseBuilder::new(self, model, input)
    }

    /// Retrieve a response by ID
    pub async fn get_response(&self, response_id: &str) -> Result<Response> {
        let path = format!("responses/{}" , response_id);
        let response = self
            .execute_with_retry(self.get(&path))
            .await?;
        
        response.json().await.map_err(Into::into)
    }

    /// Delete a response by ID
    pub async fn delete_response(&self, response_id: &str) -> Result<DeletedResponse> {
        let path = format!("responses/{}" , response_id);
        let response = self
            .execute_with_retry(self.delete(&path))
            .await?;
        
        response.json().await.map_err(Into::into)
    }

    /// Cancel an in-progress response
    pub async fn cancel_response(&self, response_id: &str) -> Result<Response> {
        let path = format!("responses/{}/cancel" , response_id);
        let response = self
            .execute_with_retry(self.post(&path))
            .await?;
        
        response.json().await.map_err(Into::into)
    }

    /// List responses with optional filtering and pagination
    pub async fn list_responses(
        &self,
        limit: Option<u32>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> Result<ResponseList> {
        let mut url = "responses".to_string();
        let mut params = Vec::new();
        
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(after) = after {
            params.push(format!("after={}", after));
        }
        if let Some(before) = before {
            params.push(format!("before={}", before));
        }
        
        if !params.is_empty() {
            url.push_str("?");
            url.push_str(&params.join("&"));
        }
        
        let response = self
            .execute_with_retry(self.get(&url))
            .await?;
        
        response.json().await.map_err(Into::into)
    }

    /// Create a response with streaming support (simplified for now)
    pub async fn create_response_stream(
        &self,
        request: &CreateResponseRequest,
    ) -> Result<Response> {
        // For now, return regular response as streaming requires more complex setup
        self.create_response(request).await
    }

    /// Wait for a response to complete with configurable polling interval
    pub async fn wait_for_response(&self, response_id: &str) -> Result<Response> {
        self.wait_for_response_with_timeout(response_id, None).await
    }

    /// Wait for a response to complete with timeout
    pub async fn wait_for_response_with_timeout(
        &self,
        response_id: &str,
        timeout: Option<std::time::Duration>,
    ) -> Result<Response> {
        let start_time = std::time::Instant::now();
        let max_duration = timeout.unwrap_or_else(|| std::time::Duration::from_secs(300)); // 5 min default
        
        loop {
            if start_time.elapsed() > max_duration {
                return Err(crate::error::OpenAIError::Timeout(format!("Response {} timed out", response_id)));
            }
            
            let response = self.get_response(response_id).await?;
            match response.status {
                crate::types::ResponseStatus::Completed => return Ok(response),
                crate::types::ResponseStatus::InProgress => {
                    tokio::time::sleep(self.config().polling_interval).await;
                    continue;
                }
                crate::types::ResponseStatus::Cancelled => 
                    return Err(crate::error::OpenAIError::Cancelled(response_id.to_string())),
                crate::types::ResponseStatus::Incomplete => 
                    return Err(crate::error::OpenAIError::Api {
                        message: format!("Response {} incomplete", response_id),
                        code: None,
                        status: None,
                    }),
            }
        }
    }
}

#[derive(Debug)]
pub struct ResponseBuilder<'a> {
    client: &'a OpenAIClient,
    request: CreateResponseRequest,
}

impl<'a> ResponseBuilder<'a> {
    pub fn new(client: &'a OpenAIClient, model: Model, input: impl Into<String>) -> Self {
        Self {
            client,
            request: CreateResponseRequest::new(model, input),
        }
    }

    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.request = self.request.with_instructions(instructions);
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.request = self.request.with_temperature(temperature);
        self
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.request = self.request.with_max_tokens(max_tokens);
        self
    }

    pub fn background(mut self, background: bool) -> Self {
        self.request = self.request.with_background(background);
        self
    }

    pub fn top_p(mut self, top_p: f64) -> Self {
        self.request = self.request.with_top_p(top_p);
        self
    }

    pub fn frequency_penalty(mut self, penalty: f64) -> Self {
        self.request = self.request.with_frequency_penalty(penalty);
        self
    }

    pub fn presence_penalty(mut self, penalty: f64) -> Self {
        self.request = self.request.with_presence_penalty(penalty);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.request = self.request.with_seed(seed);
        self
    }

    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.request = self.request.with_metadata(metadata);
        self
    }

    pub fn store(mut self, store: bool) -> Self {
        self.request = self.request.with_store(store);
        self
    }

    pub fn reasoning(mut self, effort: impl Into<String>) -> Self {
        self.request = self.request.with_reasoning(effort);
        self
    }

    pub fn truncation(mut self, type_: impl Into<String>, last_turns: Option<u32>) -> Self {
        self.request = self.request.with_truncation(type_, last_turns);
        self
    }

    pub fn modalities(mut self, modalities: Vec<String>) -> Self {
        self.request = self.request.with_modalities(modalities);
        self
    }

    pub fn response_format_text(mut self) -> Self {
        self.request = self.request.with_response_format_text();
        self
    }

    pub fn response_format_json(mut self) -> Self {
        self.request = self.request.with_response_format_json();
        self
    }

    pub fn response_format_schema(mut self, schema: serde_json::Value) -> Self {
        self.request = self.request.with_response_format_schema(schema);
        self
    }

    pub fn parallel_tool_calls(mut self, parallel: bool) -> Self {
        self.request = self.request.with_parallel_tool_calls(parallel);
        self
    }

    pub fn tool_choice_auto(mut self) -> Self {
        self.request = self.request.with_tool_choice_auto();
        self
    }

    pub fn tool_choice_required(mut self) -> Self {
        self.request = self.request.with_tool_choice_required();
        self
    }

    pub fn tool_choice_none(mut self) -> Self {
        self.request = self.request.with_tool_choice_none();
        self
    }

    pub fn tool_choice_specific(mut self, name: impl Into<String>) -> Self {
        self.request = self.request.with_tool_choice_specific(name);
        self
    }

    pub async fn send(self) -> Result<Response> {
        self.client.create_response(&self.request).await
    }

    pub async fn send_and_wait(self) -> Result<Response> {
        let response = self.client.create_response(&self.request).await?;
        
        if response.status == crate::types::ResponseStatus::InProgress {
            self.client.wait_for_response(&response.id).await
        } else {
            Ok(response)
        }
    }

    pub async fn send_and_wait_with_timeout(self, timeout: std::time::Duration) -> Result<Response> {
        let response = self.client.create_response(&self.request).await?;
        
        if response.status == crate::types::ResponseStatus::InProgress {
            self.client.wait_for_response_with_timeout(&response.id, Some(timeout)).await
        } else {
            Ok(response)
        }
    }
}

#[cfg(test)]
use wiremock::{MockServer, Mock, ResponseTemplate};
#[cfg(test)]
use wiremock::matchers::{method, path, header, query_param};
#[cfg(test)]
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_response() {
        let mock_server = MockServer::start().await;
        
        let response_body = r#"{
            "id": "resp_123",
            "object": "response",
            "status": "completed",
            "output": [{"type": "message", "content": [{"type": "text", "text": "Hello"}]}],
            "usage": {"total_tokens": 10}
        }"#;

        Mock::given(method("POST"))
            .and(path("/responses"))
            .and(header("Authorization", "Bearer test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
            .mount(&mock_server)
            .await;

        let config = crate::config::OpenAIConfig::new("test-key")
            .with_base_url(mock_server.uri());
        let client = crate::client::OpenAIClient::new(config).unwrap();

        let request = CreateResponseRequest::new(Model::Gpt4_1Nano, "Hello");
        let response = client.create_response(&request).await.unwrap();
        
        assert_eq!(response.id, "resp_123");
        assert!(response.is_completed());
    }

    #[tokio::test]
    async fn test_get_response() {
        let mock_server = MockServer::start().await;
        
        let response_body = r#"{
            "id": "resp_123",
            "object": "response",
            "status": "completed"
        }"#;

        Mock::given(method("GET"))
            .and(path("/responses/resp_123"))
            .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
            .mount(&mock_server)
            .await;

        let config = crate::config::OpenAIConfig::new("test-key")
            .with_base_url(mock_server.uri());
        let client = crate::client::OpenAIClient::new(config).unwrap();

        let response = client.get_response("resp_123").await.unwrap();
        assert_eq!(response.id, "resp_123");
    }

    #[tokio::test]
    async fn test_delete_response() {
        let mock_server = MockServer::start().await;
        
        let response_body = r#"{
            "id": "resp_123",
            "object": "response.deleted",
            "deleted": true
        }"#;

        Mock::given(method("DELETE"))
            .and(path("/responses/resp_123"))
            .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
            .mount(&mock_server)
            .await;

        let config = crate::config::OpenAIConfig::new("test-key")
            .with_base_url(mock_server.uri());
        let client = crate::client::OpenAIClient::new(config).unwrap();

        let result = client.delete_response("resp_123").await.unwrap();
        assert!(result.deleted);
    }

    #[tokio::test]
    async fn test_list_responses() {
        let mock_server = MockServer::start().await;
        
        let response_body = r#"{
            "object": "list",
            "data": [
                {
                    "id": "resp_123",
                    "object": "response",
                    "status": "completed"
                },
                {
                    "id": "resp_124",
                    "object": "response",
                    "status": "in_progress"
                }
            ],
            "has_more": true
        }"#;

        Mock::given(method("GET"))
            .and(path("/responses"))
            .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
            .mount(&mock_server)
            .await;

        let config = crate::config::OpenAIConfig::new("test-key")
            .with_base_url(mock_server.uri());
        let client = crate::client::OpenAIClient::new(config).unwrap();

        let response_list = client.list_responses(Some(10), None, None).await.unwrap();
        assert_eq!(response_list.data.len(), 2);
        assert!(response_list.has_more.unwrap_or(false));
    }

    #[tokio::test]
    async fn test_list_responses_with_pagination() {
        let mock_server = MockServer::start().await;
        
        let response_body = r#"{
            "object": "list",
            "data": [
                {
                    "id": "resp_125",
                    "object": "response",
                    "status": "completed"
                }
            ]
        }"#;

        Mock::given(method("GET"))
            .and(path("/responses"))
            .and(query_param("limit", "5"))
            .and(query_param("after", "resp_120"))
            .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
            .mount(&mock_server)
            .await;

        let config = crate::config::OpenAIConfig::new("test-key")
            .with_base_url(mock_server.uri());
        let client = crate::client::OpenAIClient::new(config).unwrap();

        let response_list = client.list_responses(Some(5), Some("resp_120"), None).await.unwrap();
        assert_eq!(response_list.data.len(), 1);
    }

    #[tokio::test]
    async fn test_wait_for_response_timeout() {
        let mock_server = MockServer::start().await;
        
        let in_progress_body = r#"{
            "id": "resp_123",
            "object": "response",
            "status": "in_progress"
        }"#;

        Mock::given(method("GET"))
            .and(path("/responses/resp_123"))
            .respond_with(ResponseTemplate::new(200).set_body_string(in_progress_body))
            .mount(&mock_server)
            .await;

        let config = crate::config::OpenAIConfig::new("test-key")
            .with_base_url(mock_server.uri())
            .with_polling_interval(Duration::from_millis(10));
        let client = crate::client::OpenAIClient::new(config).unwrap();

        let result = client.wait_for_response_with_timeout("resp_123", Some(Duration::from_millis(50))).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), crate::error::OpenAIError::Timeout(_)));
    }
}