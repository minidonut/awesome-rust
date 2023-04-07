use app_config::AppConfig;
use app_config_manager::AppConfigManager;
use clap::{arg, Arg, Command};

mod app_config;
mod app_config_manager;

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
                        .value_parser(AppConfig::parse_repo_path)
                        .help("Set REPO_PATH value"),
                )
                .arg(
                    Arg::new("github-token")
                        .long("github-token")
                        .value_name("GITHUB_TOKEN")
                        .value_parser(AppConfig::parse_github_token)
                        .help("Set GITHUB_TOKEN value"),
                ),
        );

    let matches = app.get_matches();
    let mut app_config_manager = AppConfigManager::new();

    if let Some(config_matches) = matches.subcommand_matches("config") {
        if config_matches.get_flag("show") {
            println!("Config location: {:?}", app_config_manager.config_path);
            println!("Config values: {:?}", app_config_manager.config);
        }

        if config_matches.get_flag("verify") {
            // Implement your verification logic here
            println!("Configuration verification is not implemented yet");
        }

        if let Some(repo_path) = config_matches.get_one::<String>("repo-path") {
            app_config_manager.config.REPO_PATH = repo_path.to_string();
            println!("Saving REPO_PATH to config");
            app_config_manager.save_config();
        }

        if let Some(github_token) = config_matches.get_one::<String>("github-token") {
            app_config_manager.config.GITHUB_TOKEN = github_token.to_string();
            println!("Saving GITHUB_TOKEN to config");
            app_config_manager.save_config();
        }
    }
}
