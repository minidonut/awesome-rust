use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub GITHUB_TOKEN: String,
    pub REPO_PATH: String,
}

impl AppConfig {
    pub fn parse_repo_path(repo_path: &str) -> Result<String, String> {
        if PathBuf::from(repo_path).is_absolute() {
            Ok(repo_path.to_string())
        } else {
            Err(String::from("Repo-path must be an absolute path"))
        }
    }

    pub fn parse_github_token(github_token: &str) -> Result<String, String> {
        if github_token.starts_with("ghp_") {
            Ok(github_token.to_string())
        } else {
            Err(String::from("GitHub token must start with 'ghp_'"))
        }
    }

    pub fn data_path(&self) -> String {
        let data_file = Path::new(&self.REPO_PATH)
            .join("data/git.json")
            .to_path_buf();
        let data_file_path = data_file.to_str();
        data_file_path.unwrap().to_owned()
    }
}
