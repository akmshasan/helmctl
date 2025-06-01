# Helmctl 🚀

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Kubernetes](https://img.shields.io/badge/kubernetes-%23326ce5.svg?style=for-the-badge&logo=kubernetes&logoColor=white)](https://kubernetes.io/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![GitHub Stars](https://img.shields.io/github/stars/akmshasan/helmctl?style=for-the-badge)](https://github.com/akmshasan/helmctl/stargazers)

> 🚀 Enterprise-grade CLI for Helmfile operations and Kubernetes deployments - Built with Rust

## ✨ Features

- 🔍 **Advanced Helmfile Linting** - Environment-specific validation with strict mode
- 🚀 **Safe Deployments** - Production confirmations, dry-run modes, and rollback capabilities
- ☸️ **Kubernetes Integration** - Direct manifest deployment with multi-cluster support
- 🔄 **Rollback System** - Individual release or full environment rollback functionality
- 📊 **Status Monitoring** - Detailed release and pod status checking
- ✅ **Template Validation** - YAML syntax checking and Helmfile template rendering
- ⚙️ **Configuration Management** - Persistent settings with YAML configuration
- 🔀 **Multi-cluster Support** - Context switching with connectivity tests
- 📝 **Comprehensive Logging** - Structured JSON logging with audit trails
- 🛡️ **Safety First** - Production deployment confirmations and dry-run modes

## 🎯 Quick Start

### Prerequisites

Ensure you have the following tools installed:
- `helmfile` - [Install Helmfile](https://github.com/helmfile/helmfile#installation)
- `helm` - [Install Helm](https://helm.sh/docs/intro/install/)
- `kubectl` - [Install kubectl](https://kubernetes.io/docs/tasks/tools/)

### Installation

#### Option 1: Build from Source
```bash
# Clone or extract the project
git clone  https://github.com/akmshasan/helmctl
cd helmctl

# Build the project
make build

# Install system-wide (optional)
make install
```

#### Option 2: Download Binary
```bash
# Download from releases (when available)
curl -L -o helmctl <download-url>
chmod +x helmctl
sudo mv helmctl /usr/local/bin/
```

### Initialize Configuration
```bash
helmctl config init
helmctl config show
```

## 📋 Commands Overview

### Core Operations

#### Lint Helmfiles
```bash
# Basic linting
helmctl lint -f helmfile.yaml -e development

# Strict mode (fail on warnings)
helmctl lint -f helmfile.yaml -e production --strict

# Template validation only
helmctl lint -f helmfile.yaml --template-only
```

#### Deploy with Helmfile
```bash
# Dry-run deployment
helmctl deploy -f helmfile.yaml -e staging --dry-run

# Production deployment with confirmation
helmctl deploy -f helmfile.yaml -e production --concurrency 3

# Deploy with diff preview
helmctl deploy -f helmfile.yaml -e development --diff

# Skip dependency updates
helmctl deploy -f helmfile.yaml -e development --skip-deps
```

#### Direct Kubernetes Deployment
```bash
# Deploy manifest with dry-run
helmctl k8s-deploy -m app.yaml --dry-run

# Deploy to specific namespace with wait
helmctl k8s-deploy -m app.yaml -n production --wait --timeout 600

# Deploy with specific context
helmctl k8s-deploy -m app.yaml --context prod-cluster
```

### Advanced Operations

#### Rollback Management
```bash
# Rollback specific release to specific revision
helmctl rollback -f helmfile.yaml -e production -r app-name --revision 2

# Rollback all releases in environment
helmctl rollback -f helmfile.yaml -e staging

# Rollback with context switching
helmctl rollback -f helmfile.yaml -e production --context prod-cluster
```

#### Status Monitoring
```bash
# Check status of all releases
helmctl status -f helmfile.yaml -e development

# Check specific release status
helmctl status -f helmfile.yaml -e staging -r redis-cache

# Detailed status with Kubernetes info
helmctl status -f helmfile.yaml -e production --detailed
```

#### Template Validation
```bash
# Validate YAML syntax only
helmctl validate -f helmfile.yaml --syntax-only

# Full template validation with rendering
helmctl validate -f helmfile.yaml -e development

# Validate with verbose output
helmctl validate -f helmfile.yaml -e staging --verbose
```

### Configuration Management

#### Basic Configuration
```bash
# Show current configuration
helmctl config show

# Initialize with defaults
helmctl config init

# Set configuration values
helmctl config set default_environment staging
helmctl config set default_concurrency 3
helmctl config set preferred_context prod-cluster

# Get configuration values
helmctl config get default_environment
```

#### Available Configuration Options
- `default_environment` - Default environment for operations
- `default_concurrency` - Default concurrency level for deployments
- `default_timeout` - Default timeout for operations (seconds)
- `auto_update_repos` - Automatically update Helm repositories
- `preferred_context` - Default Kubernetes context
- `log_level` - Logging level (debug, info, warn, error)

### Context Management

#### Kubernetes Context Operations
```bash
# List available contexts
helmctl context list

# Switch to a context
helmctl context use staging-cluster

# Show current context with details
helmctl context current
```

## ⚙️ Configuration

### Configuration File (`helmctl.yaml`)

Create a configuration file in your project directory:

```yaml
# Default settings
default_environment: development
default_concurrency: 2
default_timeout: 300
auto_update_repos: true
preferred_context: minikube
log_level: info

# Custom repositories
repositories:
  - name: bitnami
    url: https://charts.bitnami.com/bitnami
  - name: stable
    url: https://charts.helm.sh/stable
  - name: prometheus-community
    url: https://prometheus-community.github.io/helm-charts
  - name: grafana
    url: https://grafana.github.io/helm-charts
```

### Global vs Local Configuration

- **Global**: `~/.helmctl.yaml` - User-wide settings
- **Local**: `./helmctl.yaml` - Project-specific settings
- **Override**: Use `-c/--config` flag to specify custom config file

## 🛡️ Safety Features

### Production Safeguards
- **Interactive confirmations** before production deployments
- **Dry-run modes** for all destructive operations
- **Context verification** before operations
- **Strict linting** with configurable warning levels

### Error Handling
- **Dependency checking** - Validates required tools are installed
- **File validation** - Checks for file existence before operations
- **Command validation** - Verifies external commands are available
- **Graceful failures** with descriptive error messages

## 🔧 Development

### Build System (Makefile)

```bash
# Development workflow
make dev              # Full development workflow
make quality          # Run all quality checks
make test             # Run tests
make lint             # Lint code
make fmt              # Format code

# Building
make build            # Build release version
make build-optimized  # Build with maximum optimizations
make debug            # Build debug version

# Installation
make install          # Install to system PATH
make uninstall        # Remove from system

# Documentation
make docs             # Generate and open documentation

# Utilities
make clean            # Clean build artifacts
make audit            # Security audit
make update           # Update dependencies
make watch            # Watch for changes and rebuild
```

### Quality Assurance

The project includes comprehensive quality checks:
- **Clippy linting** with strict rules
- **Rustfmt formatting** enforcement
- **Unit and integration tests**
- **Security auditing** with cargo-audit
- **Documentation generation**

### Project Structure

```
helmctl/
├── Cargo.toml              # Project configuration
├── Makefile                # Build system
├── README.md               # This file
├── src/
│   ├── main.rs            # Entry point
│   ├── cli.rs             # CLI definitions
│   ├── config.rs          # Configuration management
│   ├── utils.rs           # Shared utilities
│   └── commands/          # Command implementations
│       ├── mod.rs
│       ├── lint.rs
│       ├── deploy.rs
│       ├── k8s.rs
│       ├── rollback.rs
│       ├── status.rs
│       ├── validate.rs
│       ├── config_cmd.rs
│       └── context.rs
├── tests/                 # Integration tests
├── examples/              # Example configurations
└── docs/                  # Documentation
```

## 🎯 Use Cases

### CI/CD Integration
```bash
# In your CI/CD pipeline
helmctl lint -f helmfile.yaml -e staging --strict
helmctl deploy -f helmfile.yaml -e staging --dry-run
helmctl deploy -f helmfile.yaml -e staging
helmctl status -f helmfile.yaml -e staging --detailed
```

### Development Workflow
```bash
# Local development
helmctl config set default_environment development
helmctl lint -f helmfile.yaml
helmctl deploy -f helmfile.yaml --dry-run
helmctl deploy -f helmfile.yaml
```

### Production Deployment
```bash
# Production deployment with safety checks
helmctl context use production-cluster
helmctl lint -f helmfile.yaml -e production --strict
helmctl deploy -f helmfile.yaml -e production --diff
helmctl deploy -f helmfile.yaml -e production
helmctl status -f helmfile.yaml -e production --detailed
```

### Troubleshooting
```bash
# Debugging failed deployments
helmctl status -f helmfile.yaml -e staging --detailed
helmctl validate -f helmfile.yaml -e staging --verbose
helmctl rollback -f helmfile.yaml -e staging
```

## 📊 Examples

### Basic Helmfile Structure
```yaml
# helmfile.yaml
repositories:
  - name: bitnami
    url: https://charts.bitnami.com/bitnami

environments:
  development:
    values:
      - environments/development.yaml
  production:
    values:
      - environments/production.yaml

releases:
  - name: nginx
    chart: bitnami/nginx
    version: "15.4.4"
    values:
      - values/nginx-{{ .Environment.Name }}.yaml
```

### Environment Configuration
```yaml
# environments/development.yaml
nginx:
  replicaCount: 1
  service:
    type: NodePort
  resources:
    requests:
      memory: "64Mi"
      cpu: "50m"
```

## 🆘 Troubleshooting

### Common Issues

#### Command Not Found
```bash
# Ensure required tools are installed
which helmfile helm kubectl

# Install missing tools
# Helmfile: https://github.com/helmfile/helmfile#installation
# Helm: https://helm.sh/docs/intro/install/
# kubectl: https://kubernetes.io/docs/tasks/tools/
```

#### Repository Issues
```bash
# Update Helm repositories
helm repo update

# Check repository list
helm repo list

# Add missing repositories manually
helm repo add bitnami https://charts.bitnami.com/bitnami
```

#### Context Issues
```bash
# List available contexts
kubectl config get-contexts

# Set correct context
kubectl config use-context <context-name>

# Verify connection
kubectl cluster-info
```

### Verbose Mode
Use `--verbose` flag with any command to get detailed output:
```bash
helmctl deploy -f helmfile.yaml -e development --verbose
```

### Log Files
Enable logging to file for debugging:
```bash
helmctl --log-file ./logs/helmctl.log deploy -f helmfile.yaml
```

## 🤝 Contributing

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes** following the coding standards
4. **Run quality checks**: `make quality`
5. **Test your changes**: `make test`
6. **Commit your changes**: `git commit -m 'Add amazing feature'`
7. **Push to the branch**: `git push origin feature/amazing-feature`
8. **Open a Pull Request**

### Development Guidelines
- Follow Rust best practices and idioms
- Maintain test coverage for new features
- Update documentation for user-facing changes
- Run `make quality` before submitting PRs
- Use meaningful commit messages

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Helmfile team** for the excellent Helmfile tool
- **Helm community** for the Helm package manager
- **Kubernetes community** for the orchestration platform
- **Rust community** for the amazing programming language and ecosystem

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/akmshasan/helmctl/issues)
- **Discussions**: [GitHub Discussions](https://github.com/akmshasan/helmctl/discussions)
- **Documentation**: [Wiki](https://github.com/akmshasan/helmctl/wiki)

---

**Made with ❤️ and Rust 🦀**
