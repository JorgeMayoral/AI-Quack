use std::sync::Mutex;

use serde::Deserialize;

use crate::configuration::Configuration;

#[derive(Deserialize)]
pub struct ApiKey(String);

#[tauri::command]
pub fn set_api_key(api_key: ApiKey, config: tauri::State<'_, Mutex<Configuration>>) {
    config.lock().unwrap().set_api_key(api_key.0);
    config.lock().unwrap().save_to_file().unwrap();
}
