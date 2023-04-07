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

    #[cfg(test)]
    pub fn new_with_conig_path(config_path: PathBuf) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;
    use tempfile::tempdir;

    #[test]
    fn test_new() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.json");

        let manager = AppConfigManager::new_with_conig_path(config_path.clone());

        assert_eq!(manager.config.GITHUB_TOKEN, "");
        assert_eq!(manager.config.REPO_PATH, "");

        remove_file(config_path).unwrap();
        temp_dir.close().unwrap();
    }

    #[test]
    fn test_save_config() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.json");

        let mut manager = AppConfigManager::new_with_conig_path(config_path.clone());
        manager.config.GITHUB_TOKEN = "test_token".to_string();
        manager.config.REPO_PATH = "/test/path".to_string();

        manager.save_config();

        let mut saved_config = String::new();
        File::open(config_path.clone())
            .unwrap()
            .read_to_string(&mut saved_config)
            .unwrap();

        assert!(saved_config.contains("test_token"));
        assert!(saved_config.contains("/test/path"));

        remove_file(config_path).unwrap();
        temp_dir.close().unwrap();
    }
}
