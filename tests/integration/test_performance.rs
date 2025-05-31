use assert_cmd::Command;
use std::time::{Duration, Instant};

#[test]
fn test_help_performance() {
    let start = Instant::now();

    let mut cmd = Command::cargo_bin("helmctl").unwrap();
    cmd.arg("--help");
    cmd.assert().success();

    let duration = start.elapsed();

    // Help should be very fast (under 1 second)
    assert!(duration < Duration::from_secs(1),
           "Help command took too long: {:?}", duration);
}

#[test]
fn test_config_operations_performance() {
    let start = Instant::now();

    // Multiple config operations should be fast
    for _ in 0..10 {
        let mut cmd = Command::cargo_bin("helmctl").unwrap();
        cmd.args(["config", "show"]);
        cmd.assert().success();
    }

    let duration = start.elapsed();

    // 10 config operations should take less than 5 seconds
    assert!(duration < Duration::from_secs(5),
           "Config operations took too long: {:?}", duration);
}
