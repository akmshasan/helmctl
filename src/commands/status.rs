use crate::utils::{check_command_available, set_kubectl_context};
use colored::*;
use std::path::Path;
use std::process::Command;

pub fn execute(
    file: &str,
    environment: Option<&str>,
    release: Option<&str>,
    context: Option<&str>,
    detailed: bool,
    verbose: bool,
) -> Result<(), String> {
    println!("{}", "📊 Checking release status...".cyan().bold());

    if !Path::new(file).exists() {
        return Err(format!("Helmfile not found: {}", file));
    }

    check_command_available("helmfile")?;

    if let Some(ctx) = context {
        set_kubectl_context(ctx, verbose)?;
    }

    let mut cmd = Command::new("helmfile");
    cmd.arg("-f").arg(file);

    if let Some(env) = environment {
        cmd.arg("-e").arg(env);
        println!("📋 Environment: {}", env.yellow());
    }

    if let Some(rel) = release {
        cmd.arg("--selector").arg(format!("name={}", rel));
        println!("📦 Release: {}", rel.yellow());
    }

    cmd.arg("status");

    if verbose {
        println!("🔧 Command: {:?}", cmd);
    }

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to execute helmfile status: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Status check failed:\n{}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);

    if detailed {
        println!("\n{}", "🔍 Detailed Kubernetes status:".yellow().bold());
        let mut kubectl_cmd = Command::new("kubectl");
        kubectl_cmd.args(["get", "all", "-o", "wide"]);

        if let Some(env) = environment {
            // Try to get resources from namespace if it matches environment
            kubectl_cmd.args(["-n", env]);
        }

        if verbose {
            println!("🔧 Command: {:?}", kubectl_cmd);
        }

        let _ = kubectl_cmd.status();

        // Also show pod status
        println!("\n{}", "🚀 Pod Status:".yellow().bold());
        let mut pod_cmd = Command::new("kubectl");
        pod_cmd.args(["get", "pods", "-o", "wide"]);

        if let Some(env) = environment {
            pod_cmd.args(["-n", env]);
        }

        let _ = pod_cmd.status();
    }

    Ok(())
}
