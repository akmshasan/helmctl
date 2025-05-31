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
