use crate::app_config::AppConfig;
use directories::ProjectDirs;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;
pub struct AppConfigManager {
    pub config: AppConfig,
    pub config_path: PathBuf,
}

impl AppConfigManager {
    pub fn new() -> Self {
        let config_path = AppConfigManager::get_config_path();
        let config = AppConfigManager::load_or_create_config(&config_path);
        AppConfigManager {
            config,
            config_path,
        }
    }

    fn load_or_create_config(config_path: &PathBuf) -> AppConfig {
        if !config_path.exists() {
            fs::create_dir_all(config_path.parent().unwrap())
                .expect("Failed to create config directory");
            let default_config = AppConfig {
                GITHUB_TOKEN: String::from(""),
                REPO_PATH: String::from(""),
            };
            let config_json = serde_json::to_string_pretty(&default_config)
                .expect("Failed to serialize default config");
            fs::write(&config_path, config_json).expect("Failed to write default config to file");
            return default_config;
        } else {
            let mut config_file = File::open(&config_path).expect("Failed to open config file");
            let mut config_json = String::new();
            config_file
                .read_to_string(&mut config_json)
                .expect("Failed to read config file");
            let config: AppConfig =
                serde_json::from_str(&config_json).expect("Failed to deserialize config");
            return config;
        }
    }

    pub fn save_config(&self) {
        let config_json =
            serde_json::to_string_pretty(&self.config).expect("Failed to serialize config");
        fs::write(&self.config_path, config_json).expect("Failed to write config to file");
    }

    fn get_config_path() -> PathBuf {
        let project_dirs = ProjectDirs::from("", "", "awesome-rust").unwrap();
        let config_dir = project_dirs.config_dir();
        config_dir.join("config.json")
    }
}
