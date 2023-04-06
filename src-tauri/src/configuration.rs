use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Configuration {
    api_key: Secret<String>,
}

impl Configuration {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key: Secret::new(api_key),
        }
    }

    pub fn api_key(&self) -> &Secret<String> {
        &self.api_key
    }

    pub fn load_from_file() -> Result<Self, Box<dyn std::error::Error>> {
        let config_file = std::fs::read_to_string("config.json")?;
        let config: Self = serde_json::from_str(&config_file)?;
        Ok(config)
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_file = ConfigurationFile::new(self.api_key().expose_secret().to_owned());
        let config_file = serde_json::to_string_pretty(&config_file)?;
        std::fs::write("config.json", config_file)?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct ConfigurationFile {
    api_key: String,
}

impl ConfigurationFile {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}
