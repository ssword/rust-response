#[cfg(feature = "backend-abstraction")]
use async_trait::async_trait;

use crate::error::{OpenAIError, Result};
use reqwest::{RequestBuilder, Response as HttpResponse};
use std::collections::VecDeque;
use std::sync::Mutex;

/// HTTP request representation for backend abstraction
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    DELETE,
    PUT,
    PATCH,
}

/// HTTP response representation for backend abstraction
#[derive(Debug)]
pub struct HttpResponseData {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

/// Abstract HTTP backend trait for dependency injection and testing
#[cfg(feature = "backend-abstraction")]
#[async_trait]
pub trait HttpBackend: Send + Sync {
    async fn execute_request(&self, request: HttpRequest) -> Result<HttpResponseData>;
}

/// Default reqwest-based HTTP backend
#[cfg(feature = "backend-abstraction")]
pub struct ReqwestBackend {
    client: reqwest::Client,
}

#[cfg(feature = "backend-abstraction")]
impl ReqwestBackend {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

#[cfg(feature = "backend-abstraction")]
#[async_trait]
impl HttpBackend for ReqwestBackend {
    async fn execute_request(&self, request: HttpRequest) -> Result<HttpResponseData> {
        let mut req_builder = match request.method {
            HttpMethod::GET => self.client.get(&request.url),
            HttpMethod::POST => self.client.post(&request.url),
            HttpMethod::DELETE => self.client.delete(&request.url),
            HttpMethod::PUT => self.client.put(&request.url),
            HttpMethod::PATCH => self.client.patch(&request.url),
        };

        // Add headers
        for (key, value) in &request.headers {
            req_builder = req_builder.header(key, value);
        }

        // Add body if present
        if let Some(body) = &request.body {
            req_builder = req_builder.body(body.clone());
        }

        let response = req_builder.send().await.map_err(OpenAIError::Http)?;

        let status = response.status().as_u16();
        let headers: Vec<(String, String)> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let body = response.bytes().await.map_err(OpenAIError::Http)?.to_vec();

        Ok(HttpResponseData {
            status,
            headers,
            body,
        })
    }
}

/// Mock HTTP backend for testing
#[cfg(feature = "backend-abstraction")]
pub struct MockBackend {
    responses: Mutex<VecDeque<Result<HttpResponseData>>>,
    requests: Mutex<Vec<HttpRequest>>,
}

#[cfg(feature = "backend-abstraction")]
impl MockBackend {
    pub fn new() -> Self {
        Self {
            responses: Mutex::new(VecDeque::new()),
            requests: Mutex::new(Vec::new()),
        }
    }

    /// Add a response to be returned by the next request
    pub fn add_response(&self, response: Result<HttpResponseData>) {
        self.responses.lock().unwrap().push_back(response);
    }

    /// Add a successful response
    pub fn add_json_response(&self, status: u16, json: &str) {
        let response = HttpResponseData {
            status,
            headers: vec![("content-type".to_string(), "application/json".to_string())],
            body: json.as_bytes().to_vec(),
        };
        self.add_response(Ok(response));
    }

    /// Add an error response
    pub fn add_error(&self, error: OpenAIError) {
        self.add_response(Err(error));
    }

    /// Get all recorded requests
    pub fn requests(&self) -> Vec<HttpRequest> {
        self.requests.lock().unwrap().clone()
    }

    /// Clear all recorded requests
    pub fn clear_requests(&self) {
        self.requests.lock().unwrap().clear();
    }

    /// Get the last request
    pub fn last_request(&self) -> Option<HttpRequest> {
        self.requests.lock().unwrap().last().cloned()
    }
}

#[cfg(feature = "backend-abstraction")]
impl Default for MockBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "backend-abstraction")]
#[async_trait]
impl HttpBackend for MockBackend {
    async fn execute_request(&self, request: HttpRequest) -> Result<HttpResponseData> {
        // Record the request
        self.requests.lock().unwrap().push(request);

        // Return the next queued response
        self.responses
            .lock()
            .unwrap()
            .pop_front()
            .unwrap_or_else(|| {
                Err(OpenAIError::Api {
                    message: "No mock response available".to_string(),
                    code: None,
                    status: None,
                })
            })
    }
}

/// Helper for building mock responses
#[cfg(feature = "backend-abstraction")]
pub struct MockResponseBuilder {
    status: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

#[cfg(feature = "backend-abstraction")]
impl MockResponseBuilder {
    pub fn new(status: u16) -> Self {
        Self {
            status,
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    pub fn json(mut self, json: &str) -> Self {
        self.headers
            .push(("content-type".to_string(), "application/json".to_string()));
        self.body = json.as_bytes().to_vec();
        self
    }

    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self
    }

    pub fn build(self) -> HttpResponseData {
        HttpResponseData {
            status: self.status,
            headers: self.headers,
            body: self.body,
        }
    }
}

/// Extension trait for creating mock responses
#[cfg(feature = "backend-abstraction")]
pub trait MockResponseExt {
    fn mock_response(status: u16) -> MockResponseBuilder;
}

#[cfg(feature = "backend-abstraction")]
impl MockResponseExt for MockBackend {
    fn mock_response(status: u16) -> MockResponseBuilder {
        MockResponseBuilder::new(status)
    }
}

/// Convert reqwest RequestBuilder to HttpRequest
pub fn request_builder_to_http_request(builder: RequestBuilder) -> Result<HttpRequest> {
    // Note: This is a simplified conversion. In practice, you'd need to extract
    // method, URL, headers, and body from the RequestBuilder, which requires
    // some introspection that reqwest doesn't directly support.
    
    // For now, return a placeholder - in a real implementation, you'd need
    // to redesign the client to build HttpRequest directly
    Ok(HttpRequest {
        method: HttpMethod::POST,
        url: "placeholder".to_string(),
        headers: vec![],
        body: None,
    })
}

#[cfg(test)]
#[cfg(feature = "backend-abstraction")]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_backend() {
        let backend = MockBackend::new();
        
        // Add a mock response
        let mock_response = MockBackend::mock_response(200)
            .json(r#"{"id": "resp_123", "status": "completed"}"#)
            .build();
        
        backend.add_response(Ok(mock_response));
        
        // Make a request
        let request = HttpRequest {
            method: HttpMethod::POST,
            url: "https://api.openai.com/v1/responses".to_string(),
            headers: vec![
                ("authorization".to_string(), "Bearer test-key".to_string()),
                ("content-type".to_string(), "application/json".to_string()),
            ],
            body: Some(b"{}".to_vec()),
        };
        
        let response = backend.execute_request(request.clone()).await.unwrap();
        assert_eq!(response.status, 200);
        
        // Check that request was recorded
        let recorded_requests = backend.requests();
        assert_eq!(recorded_requests.len(), 1);
        assert_eq!(recorded_requests[0].method, HttpMethod::POST);
        assert_eq!(recorded_requests[0].url, "https://api.openai.com/v1/responses");
    }

    #[test]
    fn test_mock_response_builder() {
        let response = MockBackend::mock_response(201)
            .header("x-custom", "test")
            .json(r#"{"success": true}"#)
            .build();
            
        assert_eq!(response.status, 201);
        assert_eq!(response.headers.len(), 2); // content-type + x-custom
        assert_eq!(
            String::from_utf8(response.body).unwrap(),
            r#"{"success": true}"#
        );
    }

    #[tokio::test]
    async fn test_mock_backend_multiple_responses() {
        let backend = MockBackend::new();
        
        // Queue multiple responses
        backend.add_json_response(200, r#"{"id": "resp_1"}"#);
        backend.add_json_response(201, r#"{"id": "resp_2"}"#);
        
        let request = HttpRequest {
            method: HttpMethod::GET,
            url: "test".to_string(),
            headers: vec![],
            body: None,
        };
        
        // First request
        let response1 = backend.execute_request(request.clone()).await.unwrap();
        assert_eq!(response1.status, 200);
        
        // Second request
        let response2 = backend.execute_request(request.clone()).await.unwrap();
        assert_eq!(response2.status, 201);
        
        // Third request should fail (no more responses)
        let response3 = backend.execute_request(request).await;
        assert!(response3.is_err());
    }
}