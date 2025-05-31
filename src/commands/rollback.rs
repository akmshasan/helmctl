use crate::utils::{check_command_available, set_kubectl_context};
use colored::*;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

pub fn execute(
    file: &str,
    environment: Option<&str>,
    release: Option<&str>,
    revision: Option<u32>,
    context: Option<&str>,
    verbose: bool,
) -> Result<(), String> {
    println!("{}", "🔄 Rolling back Helmfile releases...".cyan().bold());

    if !Path::new(file).exists() {
        return Err(format!("Helmfile not found: {}", file));
    }

    check_command_available("helmfile")?;
    check_command_available("helm")?;

    if let Some(ctx) = context {
        set_kubectl_context(ctx, verbose)?;
    }

    // Interactive confirmation
    print!(
        "⚠️  You are about to rollback releases. This action cannot be undone. Continue? (y/N): "
    );
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if !input.trim().to_lowercase().starts_with('y') {
        return Err("Rollback cancelled by user".to_string());
    }

    // For individual release rollback with specific revision
    if let (Some(rel), Some(rev)) = (release, revision) {
        println!("🔄 Rolling back {} to revision {}...", rel, rev);
        let mut helm_cmd = Command::new("helm");
        helm_cmd.args(["rollback", rel, &rev.to_string()]);

        if verbose {
            println!("🔧 Command: {:?}", helm_cmd);
        }

        let status = helm_cmd
            .status()
            .map_err(|e| format!("Failed to execute helm rollback: {}", e))?;

        if !status.success() {
            return Err("Helm rollback failed".to_string());
        }
    } else {
        // Use helmfile's destroy and redeploy approach
        println!("🗑️  Destroying current releases...");

        // Create destroy command
        let mut destroy_cmd = Command::new("helmfile");
        destroy_cmd.arg("-f").arg(file);

        if let Some(env) = environment {
            destroy_cmd.arg("-e").arg(env);
            println!("📋 Environment: {}", env.yellow());
        }

        if let Some(rel) = release {
            destroy_cmd.arg("--selector").arg(format!("name={}", rel));
            println!("📦 Release: {}", rel.yellow());
        }

        destroy_cmd.arg("destroy");

        if verbose {
            println!("🔧 Command: {:?}", destroy_cmd);
        }

        let status = destroy_cmd
            .status()
            .map_err(|e| format!("Failed to destroy releases: {}", e))?;

        if !status.success() {
            return Err("Failed to destroy releases".to_string());
        }

        println!("🚀 Redeploying from helmfile...");

        // Create redeploy command
        let mut redeploy_cmd = Command::new("helmfile");
        redeploy_cmd.arg("-f").arg(file);

        if let Some(env) = environment {
            redeploy_cmd.arg("-e").arg(env);
        }

        if let Some(rel) = release {
            redeploy_cmd.arg("--selector").arg(format!("name={}", rel));
        }

        redeploy_cmd.arg("sync");

        if verbose {
            println!("🔧 Command: {:?}", redeploy_cmd);
        }

        let status = redeploy_cmd
            .status()
            .map_err(|e| format!("Failed to redeploy: {}", e))?;

        if !status.success() {
            return Err("Failed to redeploy releases".to_string());
        }
    }

    println!("{}", "✅ Rollback completed".green());
    Ok(())
}
