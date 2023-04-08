use std::sync::Mutex;

use crate::{configuration::Configuration, open_ai_client::HttpClient};

#[tracing::instrument(name = "Getting text response from OpenAI", skip(config, client))]
#[tauri::command]
pub async fn get_text_response(
    user_prompt: String,
    config: tauri::State<'_, Mutex<Configuration>>,
    client: tauri::State<'_, HttpClient>,
) -> Result<String, ()> {
    let configuration = config.lock().unwrap().clone();
    let response = match client.get_ai_response(configuration, user_prompt).await {
        Ok(response) => response,
        Err(e) => {
            tracing::error!("Error getting response from OpenAI: {}", e);
            "Error getting response, try again later".to_owned()
        }
    };
    Ok(response)
}
