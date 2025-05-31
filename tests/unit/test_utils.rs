use helmctl::utils::{log_operation, check_command_available};
use std::process::Command;

#[test]
fn test_log_operation_success() {
    // This test just ensures the function doesn't panic
    log_operation("test", "test details", true);
    log_operation("test", "test details", false);
}

#[test]
fn test_check_command_available_existing() {
    // Test with a command that should always exist
    let result = check_command_available("echo");
    assert!(result.is_ok());
}

#[test]
fn test_check_command_available_nonexistent() {
    // Test with a command that definitely doesn't exist
    let result = check_command_available("definitelynonexistentcommand12345");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[test]
fn test_command_execution() {
    // Test basic command execution
    let output = Command::new("echo")
        .arg("test")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "test");
}
