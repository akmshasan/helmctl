use crate::utils::check_command_available;
use colored::*;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn execute(
    file: &str,
    environment: Option<&str>,
    syntax_only: bool,
    verbose: bool,
) -> Result<(), String> {
    println!(
        "{}",
        format!("🔍 Validating Helmfile templates: {}", file)
            .cyan()
            .bold()
    );

    if !Path::new(file).exists() {
        return Err(format!("Helmfile not found: {}", file));
    }

    if syntax_only {
        validate_yaml_syntax(file)?;
    } else {
        validate_templates(file, environment, verbose)?;
    }

    Ok(())
}

fn validate_yaml_syntax(file: &str) -> Result<(), String> {
    println!("🔍 Validating YAML syntax...");

    match fs::read_to_string(file) {
        Ok(content) => {
            // Split by document separator and validate each document
            let documents: Vec<&str> = content.split("---").collect();

            for (i, doc) in documents.iter().enumerate() {
                let trimmed = doc.trim();
                if trimmed.is_empty() {
                    continue;
                }

                match serde_yaml::from_str::<serde_yaml::Value>(trimmed) {
                    Ok(_) => {
                        if documents.len() > 1 {
                            println!("✅ Document {} syntax is valid", i + 1);
                        }
                    }
                    Err(e) => {
                        return Err(format!("YAML syntax error in document {}: {}", i + 1, e));
                    }
                }
            }

            println!("{}", "✅ All YAML syntax is valid".green());
            Ok(())
        }
        Err(e) => Err(format!("Failed to read file: {}", e)),
    }
}

fn validate_templates(file: &str, environment: Option<&str>, verbose: bool) -> Result<(), String> {
    check_command_available("helmfile")?;

    println!("🔍 Validating templates with rendering...");

    let mut cmd = Command::new("helmfile");
    cmd.arg("-f").arg(file);

    if let Some(env) = environment {
        cmd.arg("-e").arg(env);
        println!("📋 Environment: {}", env.yellow());
    }

    cmd.args(["template", "--skip-deps"]);

    if verbose {
        println!("🔧 Command: {:?}", cmd);
    }

    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to execute helmfile template: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Template validation failed:\n{}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check if the output contains valid YAML
    if !stdout.trim().is_empty() {
        // Try to parse the rendered output as YAML
        match serde_yaml::from_str::<serde_yaml::Value>(&stdout) {
            Ok(_) => println!("{}", "✅ Rendered templates are valid YAML".green()),
            Err(e) => {
                if verbose {
                    println!("⚠️  Warning: Rendered output may not be valid YAML: {}", e);
                    println!(
                        "Output preview:\n{}",
                        &stdout[..std::cmp::min(500, stdout.len())]
                    );
                }
            }
        }
    }

    println!("{}", "✅ Templates validation completed".green());
    Ok(())
}
