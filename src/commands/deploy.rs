use crate::utils::{
    check_command_available, check_helm_diff_plugin, confirm_production_deployment,
    set_kubectl_context, update_helm_repos,
};
use colored::*;
use std::path::Path;
use std::process::Command;

#[allow(clippy::too_many_arguments)]
pub fn execute(
    file: &str,
    environment: Option<&str>,
    dry_run: bool,
    diff: bool,
    skip_deps: bool,
    concurrency: u8,
    context: Option<&str>,
    verbose: bool,
) -> Result<(), String> {
    println!(
        "{}",
        format!("🚀 Deploying Helmfile: {}", file).cyan().bold()
    );

    if !Path::new(file).exists() {
        return Err(format!("Helmfile not found: {}", file));
    }

    check_command_available("helmfile")?;
    check_command_available("helm")?;

    if let Some(ctx) = context {
        set_kubectl_context(ctx, verbose)?;
    }

    println!("📦 Updating Helm repositories...");
    update_helm_repos(verbose)?;

    let mut cmd = Command::new("helmfile");
    cmd.arg("-f").arg(file);

    if let Some(env) = environment {
        cmd.arg("-e").arg(env);
        println!("📋 Environment: {}", env.yellow());
    }

    if diff {
        check_helm_diff_plugin()?;
        cmd.arg("diff");
        println!("🔍 Running diff to show changes");
    } else if dry_run {
        cmd.arg("sync");
        cmd.args(["--args", "--dry-run"]);
        println!("🔍 Running in dry-run mode");
    } else {
        cmd.arg("sync");
    }

    if skip_deps {
        cmd.arg("--skip-deps");
        println!("⏭️  Skipping dependency update");
    }

    cmd.arg("--concurrency").arg(concurrency.to_string());
    println!("🔄 Concurrency level: {}", concurrency);

    // Safety check for production deployments
    if !dry_run && !diff {
        confirm_production_deployment(environment, context)?;
    }

    if verbose {
        println!("🔧 Command: {:?}", cmd);
    }

    println!("🚀 Running helmfile operation...");
    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute helmfile: {}", e))?;

    if !status.success() {
        return Err("Helmfile operation failed".to_string());
    }

    Ok(())
}
