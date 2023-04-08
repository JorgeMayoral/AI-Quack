use std::sync::Mutex;

use serde::Deserialize;

use crate::{configuration::Configuration, open_ai_client::HttpClient};

#[derive(Deserialize)]
pub struct ApiKey(String);

#[tracing::instrument(name = "Setting OpenAI API key", skip(config, client, api_key))]
#[tauri::command]
pub async fn set_api_key(
    api_key: ApiKey,
    config: tauri::State<'_, Mutex<Configuration>>,
    client: tauri::State<'_, HttpClient>,
) -> Result<bool, ()> {
    tracing::info!("Setting OpenAI API key");
    config.lock().unwrap().set_api_key(api_key.0);
    let configuration = config.lock().unwrap().clone();
    tracing::info!("Validating OpenAI API key");
    let is_valid = match client.validate_api_key(configuration).await {
        Ok(true) => {
            tracing::info!("OpenAI API key is valid");
            true
        }
        _ => {
            tracing::info!("OpenAI API key is invalid");
            return Ok(false);
        }
    };
    tracing::info!("Saving configuration to file");
    config.lock().unwrap().save_to_file().unwrap();
    Ok(is_valid)
}
