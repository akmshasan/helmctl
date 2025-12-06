use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_command() {
    let mut cmd = cargo_bin!("helmctl").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("helmctl"));
}

#[test]
fn test_version_command() {
    let mut cmd = cargo_bin!("helmctl").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2.0.0"));
}

#[test]
fn test_config_commands() {
    // Test config show (should work even without config file)
    let mut cmd = cargo_bin!("helmctl").unwrap();
    cmd.args(["config", "show"]);
    cmd.assert().success();
}
