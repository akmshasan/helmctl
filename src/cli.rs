use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "helmctl")]
#[command(about = "A comprehensive CLI tool for Helmfile operations and Kubernetes deployments")]
#[command(version = "2.0.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Configuration file path
    #[arg(short, long, default_value = "helmctl.yaml")]
    pub config: String,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Log file path
    #[arg(long)]
    pub log_file: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Lint Helmfile configurations
    Lint {
        /// Path to helmfile (default: helmfile.yaml)
        #[arg(short, long, default_value = "helmfile.yaml")]
        file: String,

        /// Environment to lint against
        #[arg(short, long)]
        environment: Option<String>,

        /// Strict mode - fail on warnings
        #[arg(short, long)]
        strict: bool,

        /// Validate templates only
        #[arg(long)]
        template_only: bool,
    },
    /// Deploy using helmfile sync
    Deploy {
        /// Path to helmfile (default: helmfile.yaml)
        #[arg(short, long, default_value = "helmfile.yaml")]
        file: String,

        /// Environment to deploy to
        #[arg(short, long)]
        environment: Option<String>,

        /// Dry run mode
        #[arg(long)]
        dry_run: bool,

        /// Use diff instead of sync for dry-run (shows changes)
        #[arg(long)]
        diff: bool,

        /// Skip dependency update
        #[arg(long)]
        skip_deps: bool,

        /// Concurrency level
        #[arg(short, long, default_value = "1")]
        concurrency: u8,

        /// Kubernetes context
        #[arg(long)]
        context: Option<String>,
    },
    /// Deploy directly to Kubernetes cluster
    K8sDeploy {
        /// Kubernetes manifest file or directory
        #[arg(short, long)]
        manifest: String,

        /// Kubernetes namespace
        #[arg(short, long)]
        namespace: Option<String>,

        /// Kubernetes context
        #[arg(short, long)]
        context: Option<String>,

        /// Dry run mode
        #[arg(long)]
        dry_run: bool,

        /// Wait for deployment to complete
        #[arg(short, long)]
        wait: bool,

        /// Timeout for wait (in seconds)
        #[arg(long, default_value = "300")]
        timeout: u32,
    },
    /// Rollback a Helmfile release
    Rollback {
        /// Path to helmfile
        #[arg(short, long, default_value = "helmfile.yaml")]
        file: String,

        /// Environment
        #[arg(short, long)]
        environment: Option<String>,

        /// Release name (optional, rollback all if not specified)
        #[arg(short, long)]
        release: Option<String>,

        /// Revision to rollback to
        #[arg(long)]
        revision: Option<u32>,

        /// Kubernetes context
        #[arg(long)]
        context: Option<String>,
    },
    /// Check status of deployed releases
    Status {
        /// Path to helmfile
        #[arg(short, long, default_value = "helmfile.yaml")]
        file: String,

        /// Environment
        #[arg(short, long)]
        environment: Option<String>,

        /// Release name (optional, check all if not specified)
        #[arg(short, long)]
        release: Option<String>,

        /// Kubernetes context
        #[arg(long)]
        context: Option<String>,

        /// Show detailed status
        #[arg(long)]
        detailed: bool,
    },
    /// Validate Helmfile templates
    Validate {
        /// Path to helmfile
        #[arg(short, long, default_value = "helmfile.yaml")]
        file: String,

        /// Environment to validate against
        #[arg(short, long)]
        environment: Option<String>,

        /// Validate syntax only
        #[arg(long)]
        syntax_only: bool,
    },
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Switch Kubernetes context
    Context {
        #[command(subcommand)]
        action: ContextAction,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show current configuration
    Show,
    /// Initialize default configuration
    Init,
    /// Set configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },
    /// Get configuration value
    Get {
        /// Configuration key
        key: String,
    },
}

#[derive(Subcommand)]
pub enum ContextAction {
    /// List available contexts
    List,
    /// Switch to a context
    Use {
        /// Context name
        context: String,
    },
    /// Show current context
    Current,
}
