use clap::Parser;
use colored::*;

mod cli;
mod commands;
mod config;
mod utils;

use cli::*;
use commands::*;
use config::Config;

fn main() {
    let cli = Cli::parse();

    // Load configuration
    let config = Config::load(&cli.config).unwrap_or_default();

    // Initialize logging if specified
    if let Some(log_file) = &cli.log_file {
        utils::init_logging(log_file);
    }

    let result = match cli.command {
        Commands::Lint {
            file,
            environment,
            strict,
            template_only,
        } => {
            let env = environment.or(config.default_environment.clone());
            let result = lint::execute(&file, env.as_deref(), strict, template_only, cli.verbose);
            utils::log_operation(
                "lint",
                &format!("file: {}, env: {:?}", file, env),
                result.is_ok(),
            );
            result
        }
        Commands::Deploy {
            file,
            environment,
            dry_run,
            diff,
            skip_deps,
            concurrency,
            context,
        } => {
            let env = environment.or(config.default_environment.clone());
            let conc = if concurrency == 1 {
                config.default_concurrency.unwrap_or(1)
            } else {
                concurrency
            };
            let ctx = context.or(config.preferred_context.clone());
            let result = deploy::execute(
                &file,
                env.as_deref(),
                dry_run,
                diff,
                skip_deps,
                conc,
                ctx.as_deref(),
                cli.verbose,
            );
            utils::log_operation(
                "deploy",
                &format!("file: {}, env: {:?}, dry_run: {}", file, env, dry_run),
                result.is_ok(),
            );
            result
        }
        Commands::K8sDeploy {
            manifest,
            namespace,
            context,
            dry_run,
            wait,
            timeout,
        } => {
            let ctx = context.or(config.preferred_context.clone());
            let to = if timeout == 300 {
                config.default_timeout.unwrap_or(300)
            } else {
                timeout
            };
            let result = k8s::execute(
                &manifest,
                namespace.as_deref(),
                ctx.as_deref(),
                dry_run,
                wait,
                to,
                cli.verbose,
            );
            utils::log_operation(
                "k8s-deploy",
                &format!("manifest: {}, dry_run: {}", manifest, dry_run),
                result.is_ok(),
            );
            result
        }
        Commands::Rollback {
            file,
            environment,
            release,
            revision,
            context,
        } => {
            let env = environment.or(config.default_environment.clone());
            let ctx = context.or(config.preferred_context.clone());
            let result = rollback::execute(
                &file,
                env.as_deref(),
                release.as_deref(),
                revision,
                ctx.as_deref(),
                cli.verbose,
            );
            utils::log_operation(
                "rollback",
                &format!("file: {}, release: {:?}", file, release),
                result.is_ok(),
            );
            result
        }
        Commands::Status {
            file,
            environment,
            release,
            context,
            detailed,
        } => {
            let env = environment.or(config.default_environment.clone());
            let ctx = context.or(config.preferred_context.clone());
            let result = status::execute(
                &file,
                env.as_deref(),
                release.as_deref(),
                ctx.as_deref(),
                detailed,
                cli.verbose,
            );
            utils::log_operation(
                "status",
                &format!("file: {}, release: {:?}", file, release),
                result.is_ok(),
            );
            result
        }
        Commands::Validate {
            file,
            environment,
            syntax_only,
        } => {
            let env = environment.or(config.default_environment.clone());
            let result = validate::execute(&file, env.as_deref(), syntax_only, cli.verbose);
            utils::log_operation(
                "validate",
                &format!("file: {}, syntax_only: {}", file, syntax_only),
                result.is_ok(),
            );
            result
        }
        Commands::Config { action } => config_cmd::execute(action, &cli.config),
        Commands::Context { action } => context::execute(action),
    };

    match result {
        Ok(_) => {
            println!("{}", "✓ Operation completed successfully".green().bold());
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("{} {}", "✗ Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
}
