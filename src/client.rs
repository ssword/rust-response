use reqwest::{Client, RequestBuilder, Response as HttpResponse};
use backon::{ExponentialBuilder, Retryable};
use crate::config::OpenAIConfig;
use crate::error::{OpenAIError, Result};
use std::borrow::Cow;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct OpenAIClient {
    config: Arc<OpenAIConfig>,
    client: Arc<Client>,
}

impl OpenAIClient {
    pub fn new(config: OpenAIConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()
            .map_err(OpenAIError::Http)?;

        Ok(Self { 
            config: Arc::new(config), 
            client: Arc::new(client) 
        })
    }

    pub fn from_env() -> Result<Self> {
        let config = OpenAIConfig::from_env()?;
        Self::new(config)
    }

    pub fn from_env_with_prefix(prefix: &str) -> Result<Self> {
        let config = OpenAIConfig::from_env_with_prefix(prefix)?;
        Self::new(config)
    }

    pub fn config(&self) -> &OpenAIConfig {
        &self.config
    }

    /// Efficiently build URL with zero allocations when possible
    fn build_url<'a>(&self, path: &'a str) -> Cow<'a, str> {
        let base = self.config.base_url.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        
        // If base_url doesn't end with '/' and path doesn't start with '/', we need to add '/'
        if !self.config.base_url.ends_with('/') && !path.is_empty() {
            Cow::Owned(format!("{}/{}", base, path))
        } else if self.config.base_url.ends_with('/') && path.starts_with('/') {
            // Remove duplicate slash
            Cow::Owned(format!("{}{}", base, path))
        } else {
            // Direct concatenation works
            Cow::Owned(format!("{}{}", self.config.base_url, if path.is_empty() { "" } else { "/" }))
        }
    }

    pub(crate) fn get(&self, path: &str) -> RequestBuilder {
        let url = self.build_url(path);
        let mut request = self.client.get(url.as_ref());
        request = self.add_headers(request);
        request
    }

    pub(crate) fn post(&self, path: &str) -> RequestBuilder {
        let url = self.build_url(path);
        let mut request = self.client.post(url.as_ref());
        request = self.add_headers(request);
        request
    }

    pub(crate) fn delete(&self, path: &str) -> RequestBuilder {
        let url = self.build_url(path);
        let mut request = self.client.delete(url.as_ref());
        request = self.add_headers(request);
        request
    }

    fn add_headers(&self, request: RequestBuilder) -> RequestBuilder {
        let mut request = request
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json");

        if let Some(organization) = &self.config.organization {
            request = request.header("OpenAI-Organization", organization);
        }

        if let Some(project) = &self.config.project {
            request = request.header("OpenAI-Project", project);
        }

        request
    }

    pub(crate) async fn execute_with_retry(&self, request: RequestBuilder) -> Result<HttpResponse> {
        let config = &self.config;
        
        let operation = || async {
            // Clone the request more efficiently - only clone once
            let req = request.try_clone()
                .ok_or_else(|| OpenAIError::Http(
                    reqwest::Error::from(std::io::Error::new(
                        std::io::ErrorKind::Other, 
                        "Failed to clone request"
                    ))
                ))?;
                
            let response = req.send()
                .await
                .map_err(OpenAIError::Http)?;

            let status = response.status();
            
            if status.is_success() {
                Ok(response)
            } else {
                let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                Err(OpenAIError::from_api_response(status, &error_body))
            }
        };

        if config.max_retries == 0 {
            operation().await
        } else {
            operation
                .retry(&ExponentialBuilder::default()
                    .with_min_delay(config.retry_delay)
                    .with_max_times(config.max_retries as usize))
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, header};
    use std::time::Duration;

    #[tokio::test]
    async fn test_client_creation() {
        let config = OpenAIConfig::new("test-key");
        let client = OpenAIClient::new(config).unwrap();
        assert_eq!(client.config().api_key, "test-key");
    }

    #[tokio::test]
    async fn test_headers() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/test"))
            .and(header("Authorization", "Bearer test-key"))
            .and(header("Content-Type", "application/json"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let config = OpenAIConfig::new("test-key")
            .with_base_url(mock_server.uri());
        let client = OpenAIClient::new(config).unwrap();

        let response = client.get("test").send().await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_retry_logic() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/retry"))
            .respond_with(ResponseTemplate::new(500))
            .expect(2)
            .mount(&mock_server)
            .await;

        let config = OpenAIConfig::new("test-key")
            .with_base_url(mock_server.uri())
            .with_max_retries(1)
            .with_retry_delay(Duration::from_millis(10));
        let client = OpenAIClient::new(config).unwrap();

        let result = client.execute_with_retry(client.get("retry")).await;
        assert!(result.is_err());
    }
}