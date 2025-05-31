use helmctl::config::{Config, Repository};
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_config_default() {
    let config = Config::default();
    assert!(config.default_environment.is_none());
    assert!(config.default_concurrency.is_none());
    assert!(config.repositories.is_none());
}

#[test]
fn test_config_load_nonexistent_file() {
    let result = Config::load("nonexistent.yaml");
    assert!(result.is_ok());
    let config = result.unwrap();
    assert!(config.default_environment.is_none());
}

#[test]
fn test_config_save_and_load() {
    let mut temp_file = NamedTempFile::new().unwrap();
    let config_path = temp_file.path().to_str().unwrap();

    let config = Config {
        default_environment: Some("test".to_string()),
        default_concurrency: Some(2),
        default_timeout: Some(300),
        auto_update_repos: Some(true),
        preferred_context: Some("test-context".to_string()),
        log_level: Some("info".to_string()),
        repositories: Some(vec![
            Repository {
                name: "test-repo".to_string(),
                url: "https://example.com/charts".to_string(),
            }
        ]),
    };

    // Save config
    config.save(config_path).unwrap();

    // Load config
    let loaded_config = Config::load(config_path).unwrap();

    assert_eq!(loaded_config.default_environment, Some("test".to_string()));
    assert_eq!(loaded_config.default_concurrency, Some(2));
    assert_eq!(loaded_config.preferred_context, Some("test-context".to_string()));
    assert!(loaded_config.repositories.is_some());
    assert_eq!(loaded_config.repositories.unwrap().len(), 1);
}

#[test]
fn test_config_default_config() {
    let config = Config::default_config();

    assert_eq!(config.default_environment, Some("development".to_string()));
    assert_eq!(config.default_concurrency, Some(2));
    assert_eq!(config.default_timeout, Some(300));
    assert_eq!(config.auto_update_repos, Some(true));
    assert_eq!(config.log_level, Some("info".to_string()));
    assert!(config.repositories.is_some());

    let repos = config.repositories.unwrap();
    assert_eq!(repos.len(), 2);
    assert_eq!(repos[0].name, "bitnami");
    assert_eq!(repos[1].name, "stable");
}

#[test]
fn test_config_invalid_yaml() {
    let mut temp_file = NamedTempFile::new().unwrap();
    write!(temp_file, "invalid: yaml: content: [").unwrap();

    let result = Config::load(temp_file.path().to_str().unwrap());
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to parse config file"));
}
