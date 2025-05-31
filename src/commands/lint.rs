use crate::utils::{check_command_available, update_helm_repos};
use colored::*;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn execute(
    file: &str,
    environment: Option<&str>,
    strict: bool,
    template_only: bool,
    verbose: bool,
) -> Result<(), String> {
    println!("{}", format!("🔍 Linting Helmfile: {}", file).cyan().bold());

    if !Path::new(file).exists() {
        return Err(format!("Helmfile not found: {}", file));
    }

    check_command_available("helmfile")?;
    check_command_available("helm")?;

    if !template_only {
        println!("📦 Updating Helm repositories...");
        update_helm_repos(verbose)?;
    }

    let mut cmd = Command::new("helmfile");
    cmd.arg("-f").arg(file);

    if let Some(env) = environment {
        cmd.arg("-e").arg(env);
        println!("📋 Environment: {}", env.yellow());
    }

    if template_only {
        cmd.args(["template", "--skip-deps"]);
        println!("🔍 Validating templates only...");
    } else {
        cmd.args(["lint", "--skip-deps"]);
        println!("🔍 Running full helmfile lint...");
    }

    if verbose {
        println!("🔧 Command: {:?}", cmd);
    }

    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to execute helmfile: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Helmfile lint failed:\n{}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let warning_count = stdout.matches("WARNING").count() + stderr.matches("WARNING").count();
    let error_count = stdout.matches("ERROR").count() + stderr.matches("ERROR").count();

    if error_count > 0 {
        println!("{}", format!("❌ Found {} errors", error_count).red());
        return Err("Linting failed with errors".to_string());
    }

    if warning_count > 0 {
        println!(
            "{}",
            format!("⚠️  Found {} warnings", warning_count).yellow()
        );
        if strict {
            return Err("Linting failed due to warnings in strict mode".to_string());
        }
    }

    if verbose && !stdout.is_empty() {
        println!("Output:\n{}", stdout);
    }

    println!("{}", "✅ Helmfile lint passed".green());
    Ok(())
}
