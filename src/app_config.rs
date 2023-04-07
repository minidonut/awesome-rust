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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_repo_path_valid() {
        let valid_path = "/absolute/path";
        let result = AppConfig::parse_repo_path(valid_path);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), valid_path);
    }

    #[test]
    fn test_parse_repo_path_invalid() {
        let invalid_path = "relative/path";
        let result = AppConfig::parse_repo_path(invalid_path);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Repo-path must be an absolute path");
    }

    #[test]
    fn test_parse_github_token_valid() {
        let valid_token = "ghp_valid_token";
        let result = AppConfig::parse_github_token(valid_token);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), valid_token);
    }

    #[test]
    fn test_parse_github_token_invalid() {
        let invalid_token = "invalid_token";
        let result = AppConfig::parse_github_token(invalid_token);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "GitHub token must start with 'ghp_'");
    }

    #[test]
    fn test_data_path() {
        let config = AppConfig {
            GITHUB_TOKEN: "ghp_example_token".to_string(),
            REPO_PATH: "/example/repo/path".to_string(),
        };

        let data_path = config.data_path();
        assert_eq!(data_path, "/example/repo/path/data/git.json");
    }
}
