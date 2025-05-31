use crate::cli::ContextAction;
use crate::utils::check_command_available;
use colored::*;
use std::process::Command;

pub fn execute(action: ContextAction) -> Result<(), String> {
    check_command_available("kubectl")?;

    match action {
        ContextAction::List => list_contexts(),
        ContextAction::Use { context } => use_context(&context),
        ContextAction::Current => show_current_context(),
    }
}

fn list_contexts() -> Result<(), String> {
    println!("{}", "📋 Available Kubernetes contexts:".cyan().bold());

    let output = Command::new("kubectl")
        .args(["config", "get-contexts"])
        .output()
        .map_err(|e| format!("Failed to get contexts: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to list contexts:\n{}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parse and colorize the output
    for (i, line) in stdout.lines().enumerate() {
        if i == 0 {
            // Header line
            println!("{}", line.bright_white().bold());
        } else if line.contains('*') {
            // Current context (marked with *)
            println!("{}", line.green().bold());
        } else {
            // Other contexts
            println!("{}", line);
        }
    }

    Ok(())
}

fn use_context(context: &str) -> Result<(), String> {
    println!(
        "{}",
        format!("🔄 Switching to context: {}", context)
            .cyan()
            .bold()
    );

    let status = Command::new("kubectl")
        .args(["config", "use-context", context])
        .status()
        .map_err(|e| format!("Failed to switch context: {}", e))?;

    if !status.success() {
        return Err(format!("Failed to switch to context: {}", context));
    }

    println!("{}", format!("✅ Switched to context: {}", context).green());

    // Show some basic info about the new context
    show_context_info(context)?;

    Ok(())
}

fn show_current_context() -> Result<(), String> {
    println!("{}", "📋 Current Kubernetes context:".cyan().bold());

    let output = Command::new("kubectl")
        .args(["config", "current-context"])
        .output()
        .map_err(|e| format!("Failed to get current context: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to get current context:\n{}", stderr));
    }

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let stdout = stdout_str.trim();
    println!("{}", stdout.yellow().bold());

    // Show additional context info
    show_context_info(stdout)?;

    Ok(())
}

fn show_context_info(_context: &str) -> Result<(), String> {
    // Get cluster info
    println!("\n{}", "🔍 Context details:".yellow());

    let output = Command::new("kubectl")
        .args([
            "config",
            "view",
            "--minify",
            "--output",
            "jsonpath={.clusters[0].cluster.server}",
        ])
        .output()
        .map_err(|e| format!("Failed to get cluster info: {}", e))?;

    if output.status.success() {
        let server = String::from_utf8_lossy(&output.stdout);
        if !server.is_empty() {
            println!("  Server: {}", server.trim());
        }
    }

    // Get current namespace
    let ns_output = Command::new("kubectl")
        .args([
            "config",
            "view",
            "--minify",
            "--output",
            "jsonpath={.contexts[0].context.namespace}",
        ])
        .output()
        .map_err(|e| format!("Failed to get namespace: {}", e))?;

    if ns_output.status.success() {
        let namespace = String::from_utf8_lossy(&ns_output.stdout);
        if !namespace.trim().is_empty() {
            println!("  Namespace: {}", namespace.trim());
        } else {
            println!("  Namespace: default");
        }
    }

    // Test connectivity
    print!("  Connectivity: ");
    let test_output = Command::new("kubectl")
        .args(["cluster-info", "--request-timeout=5s"])
        .output();

    match test_output {
        Ok(output) if output.status.success() => {
            println!("{}", "✅ Connected".green());
        }
        _ => {
            println!("{}", "❌ Connection failed".red());
        }
    }

    Ok(())
}
