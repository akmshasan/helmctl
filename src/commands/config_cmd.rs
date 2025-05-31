use crate::cli::ConfigAction;
use crate::config::Config;
use colored::*;

pub fn execute(action: ConfigAction, config_path: &str) -> Result<(), String> {
    match action {
        ConfigAction::Show => show_config(config_path),
        ConfigAction::Init => init_config(config_path),
        ConfigAction::Set { key, value } => set_config_value(config_path, &key, &value),
        ConfigAction::Get { key } => get_config_value(config_path, &key),
    }
}

fn show_config(config_path: &str) -> Result<(), String> {
    let config = Config::load(config_path)?;
    let yaml =
        serde_yaml::to_string(&config).map_err(|e| format!("Failed to serialize config: {}", e))?;

    println!("{}", "📋 Current configuration:".cyan().bold());
    println!("{}", yaml);
    Ok(())
}

fn init_config(config_path: &str) -> Result<(), String> {
    let config = Config::default_config();
    config.save(config_path)?;

    println!("{}", "✅ Configuration initialized with defaults:".green());
    println!("  • Default environment: development");
    println!("  • Default concurrency: 2");
    println!("  • Default timeout: 300s");
    println!("  • Auto update repos: true");
    println!("  • Log level: info");
    println!("  • Repositories: bitnami, stable");
    println!();
    println!("Edit {} to customize settings", config_path.yellow());

    Ok(())
}

fn set_config_value(config_path: &str, key: &str, value: &str) -> Result<(), String> {
    let mut config = Config::load(config_path)?;

    match key {
        "default_environment" => {
            config.default_environment = Some(value.to_string());
        }
        "default_concurrency" => {
            let parsed: u8 = value
                .parse()
                .map_err(|_| "Invalid concurrency value (must be 1-255)")?;
            config.default_concurrency = Some(parsed);
        }
        "default_timeout" => {
            let parsed: u32 = value
                .parse()
                .map_err(|_| "Invalid timeout value (must be a positive number)")?;
            config.default_timeout = Some(parsed);
        }
        "auto_update_repos" => {
            let parsed: bool = value
                .parse()
                .map_err(|_| "Invalid boolean value (use 'true' or 'false')")?;
            config.auto_update_repos = Some(parsed);
        }
        "preferred_context" => {
            config.preferred_context = Some(value.to_string());
        }
        "log_level" => match value.to_lowercase().as_str() {
            "debug" | "info" | "warn" | "error" => {
                config.log_level = Some(value.to_lowercase());
            }
            _ => return Err("Invalid log level (use: debug, info, warn, error)".to_string()),
        },
        _ => {
            return Err(format!(
                "Unknown configuration key: {}. Available keys: default_environment, default_concurrency, default_timeout, auto_update_repos, preferred_context, log_level", 
                key
            ));
        }
    }

    config.save(config_path)?;
    println!("{} Set {} = {}", "✅".green(), key.cyan(), value.yellow());

    Ok(())
}

fn get_config_value(config_path: &str, key: &str) -> Result<(), String> {
    let config = Config::load(config_path)?;

    let value = match key {
        "default_environment" => config
            .default_environment
            .unwrap_or_else(|| "not set".to_string()),
        "default_concurrency" => config
            .default_concurrency
            .map_or("not set".to_string(), |v| v.to_string()),
        "default_timeout" => config
            .default_timeout
            .map_or("not set".to_string(), |v| v.to_string()),
        "auto_update_repos" => config
            .auto_update_repos
            .map_or("not set".to_string(), |v| v.to_string()),
        "preferred_context" => config
            .preferred_context
            .unwrap_or_else(|| "not set".to_string()),
        "log_level" => config.log_level.unwrap_or_else(|| "not set".to_string()),
        _ => {
            return Err(format!(
                "Unknown configuration key: {}. Available keys: default_environment, default_concurrency, default_timeout, auto_update_repos, preferred_context, log_level", 
                key
            ));
        }
    };

    println!("{}: {}", key.cyan(), value.yellow());
    Ok(())
}
