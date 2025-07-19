use std::time::Duration;

#[derive(Debug, Clone)]
pub struct OpenAIConfig {
    pub api_key: String,
    pub base_url: String,
    pub organization: Option<String>,
    pub project: Option<String>,
    pub timeout: Duration,
    pub max_retries: u32,
    pub retry_delay: Duration,
    pub user_agent: String,
    pub polling_interval: Duration,
}

impl OpenAIConfig {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.openai.com/v1".to_string(),
            organization: None,
            project: None,
            timeout: Duration::from_secs(60),
            max_retries: 3,
            retry_delay: Duration::from_millis(500),
            user_agent: "openai-responses-rs/0.1.0".to_string(),
            polling_interval: Duration::from_millis(500),
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn with_organization(mut self, organization: impl Into<String>) -> Self {
        self.organization = Some(organization.into());
        self
    }

    pub fn with_project(mut self, project: impl Into<String>) -> Self {
        self.project = Some(project.into());
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn with_retry_delay(mut self, retry_delay: Duration) -> Self {
        self.retry_delay = retry_delay;
        self
    }

    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    pub fn with_polling_interval(mut self, interval: Duration) -> Self {
        self.polling_interval = interval;
        self
    }

    pub fn from_env() -> crate::error::Result<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| crate::error::OpenAIError::Configuration(
                "OPENAI_API_KEY environment variable not set".to_string()
            ))?;
        
        let mut config = Self::new(api_key);
        
        if let Ok(base_url) = std::env::var("OPENAI_BASE_URL") {
            config = config.with_base_url(base_url);
        }
        
        if let Ok(organization) = std::env::var("OPENAI_ORGANIZATION") {
            config = config.with_organization(organization);
        }
        
        if let Ok(project) = std::env::var("OPENAI_PROJECT") {
            config = config.with_project(project);
        }
        
        if let Ok(polling_interval) = std::env::var("OPENAI_POLLING_INTERVAL_MS") {
            if let Ok(ms) = polling_interval.parse::<u64>() {
                config = config.with_polling_interval(Duration::from_millis(ms));
            }
        }
        
        Ok(config)
    }

    pub fn from_env_with_prefix(prefix: &str) -> crate::error::Result<Self> {
        let api_key_var = format!("{}_API_KEY", prefix);
        let base_url_var = format!("{}_BASE_URL", prefix);
        let org_var = format!("{}_ORGANIZATION", prefix);
        let project_var = format!("{}_PROJECT", prefix);
        
        let api_key = std::env::var(&api_key_var)
            .map_err(|_| crate::error::OpenAIError::Configuration(
                format!("{} environment variable not set", api_key_var)
            ))?;
        
        let mut config = Self::new(api_key);
        
        if let Ok(base_url) = std::env::var(base_url_var) {
            config = config.with_base_url(base_url);
        }
        
        if let Ok(organization) = std::env::var(org_var) {
            config = config.with_organization(organization);
        }
        
        if let Ok(project) = std::env::var(project_var) {
            config = config.with_project(project);
        }
        
        let polling_interval_var = format!("{}_POLLING_INTERVAL_MS", prefix);
        if let Ok(polling_interval) = std::env::var(polling_interval_var) {
            if let Ok(ms) = polling_interval.parse::<u64>() {
                config = config.with_polling_interval(Duration::from_millis(ms));
            }
        }
        
        Ok(config)
    }
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self::new("dummy-key")
    }
}