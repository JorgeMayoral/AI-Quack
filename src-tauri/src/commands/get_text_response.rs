use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

use crate::configuration::Configuration;

#[derive(Debug, Deserialize)]
struct ResponseBody {
    id: String,
    object: String,
    created: u32,
    model: String,
    usage: Usage,
    choices: Vec<Choice>,
}

impl ResponseBody {
    pub fn new(
        id: String,
        object: String,
        created: u32,
        model: String,
        usage: Usage,
        choices: Vec<Choice>,
    ) -> Self {
        Self {
            id,
            object,
            created,
            model,
            usage,
            choices,
        }
    }
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

impl Usage {
    pub fn new(prompt_tokens: u32, completion_tokens: u32, total_tokens: u32) -> Self {
        Self {
            prompt_tokens,
            completion_tokens,
            total_tokens,
        }
    }
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
    finish_reason: String,
    index: u32,
}

impl Choice {
    pub fn new(message: Message, finish_reason: String, index: u32) -> Self {
        Self {
            message,
            finish_reason,
            index,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    role: String,
    content: String,
}

impl Message {
    fn new(role: String, content: String) -> Self {
        Self { role, content }
    }
}

#[derive(Debug, Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
}

impl RequestBody {
    fn new(message: String) -> Self {
        let message = Message::new("user".into(), message);
        let mut messages = generate_setup_prompts();
        messages.push(message);

        Self {
            model: "gpt-3.5-turbo".into(),
            messages,
        }
    }
}

#[tauri::command]
pub async fn get_text_response(
    user_prompt: String,
    config: tauri::State<'_, Configuration>,
) -> Result<String, ()> {
    let api_key = config.api_key().clone();
    let response = match make_request(user_prompt, api_key).await {
        Ok(response) => response,
        Err(error) => {
            tracing::error!("Error while making request to OpenAI API: {}", error);
            return Ok("ERROR".into());
        }
    };
    let text = response.choices[0].message.content.clone();
    Ok(text)
}

async fn make_request(
    user_prompt: String,
    api_key: Secret<String>,
) -> Result<ResponseBody, reqwest::Error> {
    let body = RequestBody::new(user_prompt);
    tracing::info!("Sending request to OpenAI API");
    let response = reqwest::Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            &format!("Bearer {}", api_key.expose_secret()),
        )
        .json(&serde_json::json!(body))
        .send()
        .await?
        .error_for_status()?
        .json::<ResponseBody>()
        .await?;

    tracing::info!("Request to OpenAI API completed");
    tracing::debug!("Response: {:?}", response);
    Ok(response)
}

fn generate_setup_prompts() -> Vec<Message> {
    let setup_prompt = Message::new("system".into(),
r#"
        Your name is Quacker.
        You are a helpful assistant.
        You should sound technical and professional.
        Your mission is to help programmers finding failures in the logical approach to a problem.
        You will NOT ask for code, programming languages or technologies used.
        You will NOT give code snippets as a response.
        You ONLY will try to find in the programmers logic.
        You can ask for more information if more context is needed or if the logical approach given is not detailed enough.
        Your answers should be to the point, do not digress.
        "#.into()
    );
    let user_example_prompt = Message::new("user".into(), "I am trying to get a list of users that logged in the app the past month, I query the database filtering for users that do not logged in this month, but it still gives me all the users.".into());
    let system_example_prompt = Message::new("system".into(), "Maybe the problem is caused because you are missing the lower limit in the date for your query, you are getting all the users that logged in before this month.".into());

    vec![setup_prompt, user_example_prompt, system_example_prompt]
}
