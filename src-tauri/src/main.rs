// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commands::get_text_response;
use configuration::Configuration;

mod commands;
mod configuration;

fn main() {
    let configuration = match Configuration::load_from_file() {
        Ok(configuration) => configuration,
        Err(_) => {
            let configuration = Configuration::new("".to_owned());
            configuration.save_to_file().unwrap();
            configuration
        }
    };

    tauri::Builder::default()
        .manage(configuration)
        .invoke_handler(tauri::generate_handler![get_text_response])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
