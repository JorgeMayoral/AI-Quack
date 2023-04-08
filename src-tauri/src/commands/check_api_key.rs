use std::sync::Mutex;

use serde::Serialize;

use crate::configuration::Configuration;

#[derive(Serialize)]
pub struct NeedApiKey(bool);

#[tracing::instrument(name = "Checking OpenAI API key", skip(config))]
#[tauri::command]
pub fn check_api_key(config: tauri::State<'_, Mutex<Configuration>>) -> NeedApiKey {
    let has_api_key = config.lock().unwrap().has_api_key();
    tracing::info!("OpenAI API Key is setted: {has_api_key}");
    NeedApiKey(!has_api_key)
}
