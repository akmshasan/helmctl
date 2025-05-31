use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("helmctl"))
        .stdout(predicate::str::contains("Helmfile operations"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2.0.0"));
}

#[test]
fn test_config_show_empty() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_config.yaml");

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "show"]);
    cmd.assert().success();
}

#[test]
fn test_config_init() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_config.yaml");

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "init"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Configuration initialized"));

    // Verify config file was created
    assert!(config_path.exists());

    // Verify config content
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("default_environment: development"));
    assert!(content.contains("default_concurrency: 2"));
}

#[test]
fn test_config_set_and_get() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_config.yaml");

    // Initialize config first
    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "init"]);
    cmd.assert().success();

    // Set a value
    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "--config", config_path.to_str().unwrap(),
        "config", "set", "default_environment", "testing"
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Set default_environment = testing"));

    // Get the value
    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args([
        "--config", config_path.to_str().unwrap(),
        "config", "get", "default_environment"
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("default_environment: testing"));
}

#[test]
fn test_lint_missing_file() {
    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args(["lint", "-f", "nonexistent.yaml"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Helmfile not found"));
}

#[test]
fn test_validate_missing_file() {
    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args(["validate", "-f", "nonexistent.yaml"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Helmfile not found"));
}

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.arg("invalid-command");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
}

#[test]
fn test_context_list_without_kubectl() {
    // This test assumes kubectl might not be available in CI
    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.args(["context", "list"]);
    // We expect either success (if kubectl available) or specific error
    let output = cmd.output().unwrap();
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("kubectl") || stderr.contains("not found"));
    }
}
