use serde::Deserialize;

use super::message::Message;

#[derive(Debug, Deserialize)]
pub struct ResponseBody {
    id: String,
    object: String,
    created: u32,
    model: String,
    usage: Usage,
    choices: Vec<Choice>,
}

impl ResponseBody {
    pub fn get_response_content(&self) -> String {
        self.choices[0].message.content().to_owned()
    }
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    message: Message,
    finish_reason: String,
    index: u32,
}
