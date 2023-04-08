use std::sync::Mutex;

use serde::Deserialize;

use crate::{configuration::Configuration, open_ai_client::HttpClient};

#[derive(Deserialize)]
pub struct ApiKey(String);

#[tauri::command]
pub async fn set_api_key(
    api_key: ApiKey,
    config: tauri::State<'_, Mutex<Configuration>>,
    client: tauri::State<'_, HttpClient>,
) -> Result<bool, ()> {
    config.lock().unwrap().set_api_key(api_key.0);
    let configuration = config.lock().unwrap().clone();
    let is_valid = match client.validate_api_key(configuration).await {
        Ok(true) => true,
        _ => return Ok(false),
    };
    config.lock().unwrap().save_to_file().unwrap();
    Ok(is_valid)
}
