use anyhow::Context;
use secrecy::ExposeSecret;

use crate::configuration::Configuration;

use super::{request::RequestBody, response::ResponseBody};

pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn validate_api_key(&self, config: Configuration) -> anyhow::Result<bool> {
        let api_key = config.api_key().expose_secret();
        match self
            .client
            .get("https://api.openai.com/v1/models")
            .header("Authorization", &format!("Bearer {}", api_key))
            .send()
            .await
            .context("Error while sending request to OpenAI API")?
            .status()
        {
            reqwest::StatusCode::OK => Ok(true),
            _ => Ok(false),
        }
    }

    pub async fn get_ai_response(
        &self,
        config: Configuration,
        user_prompt: String,
    ) -> anyhow::Result<String> {
        let api_key = config.api_key().expose_secret();
        let body = RequestBody::new(user_prompt);
        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", api_key))
            .json(&serde_json::json!(body))
            .send()
            .await
            .context("Error while sending request to OpenAI API")?
            .error_for_status()
            .context("Open AI API returned an error")?
            .json::<ResponseBody>()
            .await
            .context("Error parsing response")?;

        Ok(response.get_response_content())
    }
}
