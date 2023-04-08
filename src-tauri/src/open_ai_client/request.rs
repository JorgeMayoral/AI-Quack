use serde::Serialize;

use super::{message::Message, prompts::generate_setup_prompts};

#[derive(Debug, Serialize)]
pub struct RequestBody {
    model: String,
    temperature: f32,
    messages: Vec<Message>,
}

impl RequestBody {
    pub fn new(message: String) -> Self {
        let message = Message::new("user".into(), message);
        let mut messages = generate_setup_prompts();
        messages.push(message);

        Self {
            model: "gpt-3.5-turbo".into(),
            temperature: 0.5,
            messages,
        }
    }
}
