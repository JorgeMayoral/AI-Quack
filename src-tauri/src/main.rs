// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use commands::check_api_key;
use commands::get_text_response;
use commands::set_api_key;
use configuration::Configuration;
use tracing::Level;

use crate::open_ai_client::HttpClient;

mod commands;
mod configuration;
mod open_ai_client;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let configuration = match Configuration::load_from_file() {
        Ok(configuration) => configuration,
        Err(e) => {
            tracing::error!("Error while loading configuration: {}", e);
            tracing::warn!("No configuration file found, creating a new one");
            let configuration = Configuration::new("".to_owned());
            configuration.save_to_file().unwrap();
            configuration
        }
    };
    let configuration = Mutex::new(configuration);
    let http_client = HttpClient::new();

    tracing::info!("Launching application");
    tauri::Builder::default()
        .manage(configuration)
        .manage(http_client)
        .invoke_handler(tauri::generate_handler![
            get_text_response,
            check_api_key,
            set_api_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
