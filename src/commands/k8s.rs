use crate::utils::{check_command_available, confirm_production_deployment, set_kubectl_context};
use colored::*;
use std::path::Path;
use std::process::Command;

pub fn execute(
    manifest: &str,
    namespace: Option<&str>,
    context: Option<&str>,
    dry_run: bool,
    wait: bool,
    timeout: u32,
    verbose: bool,
) -> Result<(), String> {
    println!(
        "{}",
        format!("☸️  Deploying to Kubernetes: {}", manifest)
            .cyan()
            .bold()
    );

    if !Path::new(manifest).exists() {
        return Err(format!("Manifest not found: {}", manifest));
    }

    check_command_available("kubectl")?;

    if let Some(ctx) = context {
        set_kubectl_context(ctx, verbose)?;
    }

    let mut cmd = Command::new("kubectl");

    if let Some(ns) = namespace {
        cmd.arg("--namespace").arg(ns);
        println!("📦 Namespace: {}", ns.yellow());
    }

    cmd.arg("apply").arg("-f").arg(manifest);

    if dry_run {
        cmd.arg("--dry-run=client");
        println!("🔍 Running in dry-run mode");
    }

    if wait {
        cmd.arg("--wait");
        cmd.arg("--timeout").arg(format!("{}s", timeout));
        println!("⏳ Waiting for deployment (timeout: {}s)", timeout);
    }

    // Safety check for production deployments
    if !dry_run {
        confirm_production_deployment(None, context)?;
    }

    if verbose {
        println!("🔧 Command: {:?}", cmd);
    }

    println!("☸️  Running kubectl apply...");
    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute kubectl: {}", e))?;

    if !status.success() {
        return Err("Kubernetes deployment failed".to_string());
    }

    Ok(())
}
