use chrono::{DateTime, Utc};
use colored::*;
use serde::Serialize;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Serialize)]
pub struct LogEntry {
    timestamp: DateTime<Utc>,
    level: String,
    command: String,
    message: String,
    success: bool,
}

pub fn init_logging(log_file: &str) {
    // Create log directory if it doesn't exist
    if let Some(parent) = Path::new(log_file).parent() {
        let _ = fs::create_dir_all(parent);
    }
}

pub fn log_operation(command: &str, details: &str, success: bool) {
    let entry = LogEntry {
        timestamp: Utc::now(),
        level: if success {
            "INFO".to_string()
        } else {
            "ERROR".to_string()
        },
        command: command.to_string(),
        message: details.to_string(),
        success,
    };

    // This could be enhanced to write to actual log file
    if let Ok(json) = serde_json::to_string(&entry) {
        eprintln!("LOG: {}", json);
    }
}

pub fn check_command_available(command: &str) -> Result<(), String> {
    Command::new("which")
        .arg(command)
        .output()
        .map_err(|_| {
            format!(
                "Command '{}' not found. Please install {} and make sure it's in your PATH.",
                command, command
            )
        })
        .and_then(|output| {
            if output.status.success() {
                Ok(())
            } else {
                Err(format!(
                    "Command '{}' not found. Please install {} and make sure it's in your PATH.",
                    command, command
                ))
            }
        })
}

pub fn update_helm_repos(verbose: bool) -> Result<(), String> {
    let repos = vec![
        ("bitnami", "https://charts.bitnami.com/bitnami"),
        ("stable", "https://charts.helm.sh/stable"),
    ];

    for (name, url) in repos {
        if verbose {
            println!("📋 Adding/updating Helm repository: {}", name);
        }
        let output = Command::new("helm")
            .args(["repo", "add", name, url])
            .output()
            .map_err(|e| format!("Failed to add helm repository {}: {}", name, e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stderr.contains("already exists") && verbose {
                println!("⚠️  Warning: Could not add repository {}: {}", name, stderr);
            }
        }
    }

    if verbose {
        println!("🔄 Updating Helm repositories...");
    }
    let mut cmd = Command::new("helm");
    cmd.args(["repo", "update"]);

    if !verbose {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to update helm repositories: {}", e))?;

    if !status.success() {
        return Err("Failed to update helm repositories".to_string());
    }

    Ok(())
}

pub fn check_helm_diff_plugin() -> Result<(), String> {
    let output = Command::new("helm")
        .args(["plugin", "list"])
        .output()
        .map_err(|e| format!("Failed to check helm plugins: {}", e))?;

    if !output.status.success() {
        return Err("Failed to check helm plugins".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.contains("diff") {
        return Err("Helm diff plugin not installed. Install it with: helm plugin install https://github.com/databus23/helm-diff".to_string());
    }

    Ok(())
}

pub fn set_kubectl_context(context: &str, verbose: bool) -> Result<(), String> {
    println!("🔄 Setting kubectl context to: {}", context.yellow());

    let mut cmd = Command::new("kubectl");
    cmd.args(["config", "use-context", context]);

    if verbose {
        println!("🔧 Command: {:?}", cmd);
    }

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to set context: {}", e))?;

    if !status.success() {
        return Err(format!("Failed to switch to context: {}", context));
    }

    Ok(())
}

pub fn confirm_production_deployment(
    environment: Option<&str>,
    context: Option<&str>,
) -> Result<(), String> {
    let is_prod = environment.is_some_and(|e| e.contains("prod"))
        || context.is_some_and(|c| c.contains("prod"));

    if is_prod {
        print!("⚠️  You are about to deploy to production. Continue? (y/N): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if !input.trim().to_lowercase().starts_with('y') {
            return Err("Deployment cancelled by user".to_string());
        }
    }

    Ok(())
}
