use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

mod helpers;
use helpers::TestEnvironment;

#[test]
fn test_lint_with_valid_helmfile() {
    let env = TestEnvironment::new();
    env.create_valid_helmfile();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "lint",
        "-f", env.helmfile_path_str(),
        "--template-only"  // Skip repo updates in tests
    ]);

    // This might fail if helmfile isn't installed, but we test the file validation
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should not fail due to file not found
    assert!(!stderr.contains("Helmfile not found"));
}

#[test]
fn test_validate_syntax_only() {
    let env = TestEnvironment::new();
    env.create_valid_helmfile();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "validate",
        "-f", env.helmfile_path_str(),
        "--syntax-only"
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("YAML syntax is valid"));
}

#[test]
fn test_validate_invalid_yaml_syntax() {
    let env = TestEnvironment::new();
    env.create_invalid_helmfile();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "validate",
        "-f", env.helmfile_path_str(),
        "--syntax-only"
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("YAML syntax error"));
}

#[test]
fn test_config_invalid_key() {
    let env = TestEnvironment::new();
    env.create_config();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "--config", env.config_path_str(),
        "config", "set", "invalid_key", "value"
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unknown configuration key"));
}

#[test]
fn test_config_invalid_concurrency_value() {
    let env = TestEnvironment::new();
    env.create_config();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "--config", env.config_path_str(),
        "config", "set", "default_concurrency", "invalid"
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid concurrency value"));
}

#[test]
fn test_verbose_mode() {
    let env = TestEnvironment::new();
    env.create_valid_helmfile();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "--verbose",
        "validate",
        "-f", env.helmfile_path_str(),
        "--syntax-only"
    ]);

    cmd.assert().success();
}

#[test]
fn test_custom_config_file() {
    let env = TestEnvironment::new();
    env.create_config();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "--config", env.config_path_str(),
        "config", "show"
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("development"));
}

#[test]
fn test_config_with_malformed_yaml() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("bad_config.yaml");

    // Write invalid YAML content
    fs::write(&config_path, "::: invalid_yaml :::").unwrap();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "show"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse config file"));
}

#[test]
fn test_config_with_empty_yaml() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("empty_config.yaml");

    // Write empty file - should work with defaults
    fs::write(&config_path, "").unwrap();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "show"]);

    cmd.assert().success();
}

#[test]
fn test_config_set_all_valid_keys() {
    let env = TestEnvironment::new();
    env.create_config();

    let test_cases = vec![
        ("default_environment", "production"),
        ("default_concurrency", "3"),
        ("default_timeout", "600"),
        ("auto_update_repos", "false"),
        ("preferred_context", "prod-cluster"),
        ("log_level", "debug"),
    ];

    for (key, value) in test_cases {
        let mut cmd = Command::cargo_bin("helmctl").unwrap();
        cmd.args([
            "--config", env.config_path_str(),
            "config", "set", key, value
        ]);

        cmd.assert()
            .success()
            .stdout(predicate::str::contains(format!("Set {} = {}", key, value)));
    }
}

#[test]
fn test_config_invalid_boolean_value() {
    let env = TestEnvironment::new();
    env.create_config();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "--config", env.config_path_str(),
        "config", "set", "auto_update_repos", "maybe"
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid boolean value"));
}

#[test]
fn test_config_invalid_log_level() {
    let env = TestEnvironment::new();
    env.create_config();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "--config", env.config_path_str(),
        "config", "set", "log_level", "invalid_level"
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid log level"));
}

#[test]
fn test_config_get_nonexistent_key() {
    let env = TestEnvironment::new();
    env.create_config();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "--config", env.config_path_str(),
        "config", "get", "nonexistent_key"
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unknown configuration key"));
}

#[test]
fn test_config_get_unset_value() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("minimal_config.yaml");

    // Create minimal config with only one field
    fs::write(&config_path, "default_environment: test").unwrap();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "--config", config_path.to_str().unwrap(),
        "config", "get", "preferred_context"
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("preferred_context: not set"));
}

#[test]
fn test_validate_with_environment_flag() {
    let env = TestEnvironment::new();
    env.create_valid_helmfile();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "validate",
        "-f", env.helmfile_path_str(),
        "--syntax-only",
        "-e", "development"
    ]);

    cmd.assert().success();
}

#[test]
fn test_lint_with_environment_flag() {
    let env = TestEnvironment::new();
    env.create_valid_helmfile();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "lint",
        "-f", env.helmfile_path_str(),
        "--template-only",
        "-e", "testing"
    ]);

    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should not fail due to file not found
    assert!(!stderr.contains("Helmfile not found"));
}

#[test]
fn test_lint_with_strict_mode() {
    let env = TestEnvironment::new();
    env.create_valid_helmfile();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "lint",
        "-f", env.helmfile_path_str(),
        "--template-only",
        "--strict"
    ]);

    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should not fail due to file not found
    assert!(!stderr.contains("Helmfile not found"));
}

#[test]
fn test_multiple_yaml_documents_validation() {
    let temp_dir = TempDir::new().unwrap();
    let helmfile_path = temp_dir.path().join("multi_doc.yaml");

    // Create a helmfile with multiple YAML documents
    let content = r#"---
repositories:
  - name: bitnami
    url: https://charts.bitnami.com/bitnami
---
environments:
  development:
    values:
      - env: development
---
releases:
  - name: test-app
    chart: bitnami/nginx
    namespace: default
"#;

    fs::write(&helmfile_path, content).unwrap();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "validate",
        "-f", helmfile_path.to_str().unwrap(),
        "--syntax-only"
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("YAML syntax is valid"));
}

#[test]
fn test_config_roundtrip() {
    let env = TestEnvironment::new();

    // Initialize config
    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args(["--config", env.config_path_str(), "config", "init"]);
    cmd.assert().success();

    // Set a value
    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "--config", env.config_path_str(),
        "config", "set", "default_environment", "staging"
    ]);
    cmd.assert().success();

    // Get the value back
    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "--config", env.config_path_str(),
        "config", "get", "default_environment"
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("default_environment: staging"));

    // Show should also contain the value
    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args(["--config", env.config_path_str(), "config", "show"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("default_environment: staging"));
}

#[test]
fn test_log_file_option() {
    let env = TestEnvironment::new();
    let log_path = env.temp_dir.path().join("test.log");

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "--log-file", log_path.to_str().unwrap(),
        "--help"
    ]);

    cmd.assert().success();

    // Note: The log file might not be created for --help, but the option should be accepted
}
