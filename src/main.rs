use clap::{arg, Arg, Command};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    GITHUB_TOKEN: String,
    REPO_PATH: String,
}

fn parse_repo_path(repo_path: &str) -> Result<String, String> {
    if PathBuf::from(repo_path).is_absolute() {
        Ok(repo_path.to_string())
    } else {
        Err(String::from("Repo-path must be an absolute path"))
    }
}

fn parse_github_token(github_token: &str) -> Result<String, String> {
    if github_token.starts_with("ghp_") {
        Ok(github_token.to_string())
    } else {
        Err(String::from("GitHub token must start with 'ghp_'"))
    }
}

fn main() {
    let app = Command::new("awesome-rust")
        .version("0.1.0")
        .author("Karl Saehun Chung <nycom13@gmail.com>")
        .about("An awesome Rust CLI application")
        .subcommand(
            Command::new("config")
                .about("Manage configuration")
                .arg(arg!(-c --show "Show config location and its values"))
                .arg(arg!(-v --verify "Verify config value is okay"))
                .arg(
                    Arg::new("repo-path")
                        .long("repo-path")
                        .value_name("REPO_PATH")
                        .value_parser(parse_repo_path)
                        .help("Set REPO_PATH value"),
                )
                .arg(
                    Arg::new("github-token")
                        .long("github-token")
                        .value_name("GITHUB_TOKEN")
                        .value_parser(parse_github_token)
                        .help("Set GITHUB_TOKEN value"),
                ),
        );

    let matches = app.get_matches();

    if let Some(config_matches) = matches.subcommand_matches("config") {
        let mut config = load_or_create_config();

        if config_matches.get_flag("show") {
            println!("Config location: {:?}", config_path());
            println!("Config values: {:?}", config);
        }

        if config_matches.get_flag("verify") {
            // Implement your verification logic here
            println!("Configuration verification is not implemented yet");
        }

        if let Some(repo_path) = config_matches.get_one::<String>("repo-path") {
            config.REPO_PATH = repo_path.to_string();
            println!("Saving REPO_PATH to config");
            save_config(&config);
        }

        if let Some(github_token) = config_matches.get_one::<String>("github-token") {
            config.GITHUB_TOKEN = github_token.to_string();
            println!("Saving GITHUB_TOKEN to config");
            save_config(&config);
        }
    }
}

fn load_or_create_config() -> AppConfig {
    let config_path = config_path();

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

fn save_config(config: &AppConfig) {
    let config_path = config_path();

    let config_json = serde_json::to_string_pretty(config).expect("Failed to serialize config");
    fs::write(&config_path, config_json).expect("Failed to write config to file");
}

fn config_path() -> PathBuf {
    let project_dirs = ProjectDirs::from("", "", "awesome-rust").unwrap();
    let config_dir = project_dirs.config_dir();
    config_dir.join("config.json")
}
