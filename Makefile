# Helmctl Makefile
# Professional build system for the Helmctl CLI

# Variables
BINARY_NAME := helmctl
VERSION := 2.0.0
BUILD_DIR := target
RELEASE_DIR := $(BUILD_DIR)/release
DEBUG_DIR := $(BUILD_DIR)/debug
INSTALL_PATH := /usr/local/bin

# Colors for output
RED := \033[31m
GREEN := \033[32m
YELLOW := \033[33m
BLUE := \033[34m
CYAN := \033[36m
RESET := \033[0m

# Default target
.DEFAULT_GOAL := help

# Help target
.PHONY: help
help: ## Show this help message
	@echo "$(CYAN)Helmctl Build System$(RESET)"
	@echo "$(YELLOW)Available targets:$(RESET)"
	@awk 'BEGIN {FS = ":.*##"} /^[a-zA-Z_-]+:.*##/ { printf "  $(GREEN)%-15s$(RESET) %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

# Check if required tools are installed
.PHONY: check-deps
check-deps: ## Check if required dependencies are installed
	@echo "$(BLUE)🔍 Checking dependencies...$(RESET)"
	@command -v cargo >/dev/null 2>&1 || { echo "$(RED)❌ Rust/Cargo not found. Please install Rust first.$(RESET)"; exit 1; }
	@command -v rustc >/dev/null 2>&1 || { echo "$(RED)❌ Rust compiler not found.$(RESET)"; exit 1; }
	@echo "$(GREEN)✅ All dependencies found$(RESET)"

# Clean build artifacts
.PHONY: clean
clean: ## Clean all build artifacts
	@echo "$(BLUE)🧹 Cleaning build artifacts...$(RESET)"
	@cargo clean
	@echo "$(GREEN)✅ Clean completed$(RESET)"

# Check code without building
.PHONY: check
check: check-deps ## Check code for errors without building
	@echo "$(BLUE)🔍 Checking code...$(RESET)"
	@cargo check --all-targets --all-features
	@echo "$(GREEN)✅ Check completed$(RESET)"

# Format code
.PHONY: fmt
fmt: ## Format code using rustfmt
	@echo "$(BLUE)✨ Formatting code...$(RESET)"
	@cargo fmt --all
	@echo "$(GREEN)✅ Code formatted$(RESET)"

# Check formatting
.PHONY: fmt-check
fmt-check: ## Check if code is properly formatted
	@echo "$(BLUE)🔍 Checking code formatting...$(RESET)"
	@cargo fmt --all -- --check
	@echo "$(GREEN)✅ Code formatting is correct$(RESET)"

# Lint code
.PHONY: lint
lint: check-deps ## Lint code using clippy
	@echo "$(BLUE)🔍 Linting code...$(RESET)"
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "$(GREEN)✅ Linting completed$(RESET)"

# Run tests
.PHONY: test
test: check-deps ## Run all tests
	@echo "$(BLUE)🧪 Running tests...$(RESET)"
	@cargo test --all-features
	@echo "$(GREEN)✅ All tests passed$(RESET)"

# Run tests with coverage
.PHONY: test-coverage
test-coverage: check-deps ## Run tests with coverage (requires cargo-tarpaulin)
	@echo "$(BLUE)🧪 Running tests with coverage...$(RESET)"
	@command -v cargo-tarpaulin >/dev/null 2>&1 || { echo "$(YELLOW)⚠️  Installing cargo-tarpaulin...$(RESET)"; cargo install cargo-tarpaulin; }
	@cargo tarpaulin --out Html --output-dir coverage
	@echo "$(GREEN)✅ Coverage report generated in coverage/$(RESET)"

# Build debug version
.PHONY: debug
debug: check-deps ## Build debug version
	@echo "$(BLUE)🔨 Building debug version...$(RESET)"
	@cargo build
	@echo "$(GREEN)✅ Debug build completed$(RESET)"
	@echo "$(CYAN)📦 Binary: $(DEBUG_DIR)/$(BINARY_NAME)$(RESET)"

# Build release version
.PHONY: build
build: check-deps ## Build optimized release version
	@echo "$(BLUE)🔨 Building release version...$(RESET)"
	@cargo build --release
	@echo "$(GREEN)✅ Release build completed$(RESET)"
	@echo "$(CYAN)📦 Binary: $(RELEASE_DIR)/$(BINARY_NAME)$(RESET)"
	@echo "$(CYAN)📏 Size: $$(du -h $(RELEASE_DIR)/$(BINARY_NAME) | cut -f1)$(RESET)"

# Build with all optimizations
.PHONY: build-optimized
build-optimized: check-deps ## Build with maximum optimizations
	@echo "$(BLUE)🔨 Building optimized release...$(RESET)"
	@RUSTFLAGS="-C target-cpu=native" cargo build --release
	@strip $(RELEASE_DIR)/$(BINARY_NAME) 2>/dev/null || true
	@echo "$(GREEN)✅ Optimized build completed$(RESET)"
	@echo "$(CYAN)📦 Binary: $(RELEASE_DIR)/$(BINARY_NAME)$(RESET)"
	@echo "$(CYAN)📏 Size: $$(du -h $(RELEASE_DIR)/$(BINARY_NAME) | cut -f1)$(RESET)"

# Install binary system-wide
.PHONY: install
install: build ## Install binary to system PATH
	@echo "$(BLUE)📦 Installing $(BINARY_NAME) to $(INSTALL_PATH)...$(RESET)"
	@sudo cp $(RELEASE_DIR)/$(BINARY_NAME) $(INSTALL_PATH)/
	@sudo chmod +x $(INSTALL_PATH)/$(BINARY_NAME)
	@echo "$(GREEN)✅ $(BINARY_NAME) installed successfully$(RESET)"
	@echo "$(CYAN)🚀 Try: $(BINARY_NAME) --help$(RESET)"

# Uninstall binary
.PHONY: uninstall
uninstall: ## Uninstall binary from system PATH
	@echo "$(BLUE)🗑️  Uninstalling $(BINARY_NAME)...$(RESET)"
	@sudo rm -f $(INSTALL_PATH)/$(BINARY_NAME)
	@echo "$(GREEN)✅ $(BINARY_NAME) uninstalled$(RESET)"

# Run the binary (debug version)
.PHONY: run
run: debug ## Run the debug version with --help
	@echo "$(BLUE)🚀 Running $(BINARY_NAME)...$(RESET)"
	@$(DEBUG_DIR)/$(BINARY_NAME) --help

# Run the binary with custom arguments
.PHONY: run-args
run-args: debug ## Run debug version with custom arguments (use ARGS="...")
	@echo "$(BLUE)🚀 Running $(BINARY_NAME) $(ARGS)...$(RESET)"
	@$(DEBUG_DIR)/$(BINARY_NAME) $(ARGS)

# Benchmark (if you have criterion benchmarks)
.PHONY: bench
bench: ## Run benchmarks
	@echo "$(BLUE)📊 Running benchmarks...$(RESET)"
	@cargo bench

# Generate documentation
.PHONY: docs
docs: ## Generate and open documentation
	@echo "$(BLUE)📚 Generating documentation...$(RESET)"
	@cargo doc --open --no-deps

# Security audit
.PHONY: audit
audit: ## Run security audit (requires cargo-audit)
	@echo "$(BLUE)🔒 Running security audit...$(RESET)"
	@command -v cargo-audit >/dev/null 2>&1 || { echo "$(YELLOW)⚠️  Installing cargo-audit...$(RESET)"; cargo install cargo-audit; }
	@cargo audit

# Update dependencies
.PHONY: update
update: ## Update dependencies
	@echo "$(BLUE)⬆️  Updating dependencies...$(RESET)"
	@cargo update
	@echo "$(GREEN)✅ Dependencies updated$(RESET)"

# Full quality check pipeline
.PHONY: quality
quality: fmt-check lint test ## Run full quality check pipeline
	@echo "$(GREEN)✅ All quality checks passed$(RESET)"

# Development workflow
.PHONY: dev
dev: clean check test build ## Complete development workflow
	@echo "$(GREEN)✅ Development workflow completed$(RESET)"

# Release workflow
.PHONY: release
release: clean quality build-optimized test ## Complete release workflow
	@echo "$(GREEN)✅ Release workflow completed$(RESET)"
	@echo "$(CYAN)🎉 Ready for release!$(RESET)"

# Package for distribution
.PHONY: package
package: build-optimized ## Package binary for distribution
	@echo "$(BLUE)📦 Packaging for distribution...$(RESET)"
	@mkdir -p dist
	@cp $(RELEASE_DIR)/$(BINARY_NAME) dist/
	@cp README.md dist/
	@tar -czf dist/$(BINARY_NAME)-$(VERSION)-$(shell uname -s)-$(shell uname -m).tar.gz -C dist $(BINARY_NAME) README.md
	@echo "$(GREEN)✅ Package created: dist/$(BINARY_NAME)-$(VERSION)-$(shell uname -s)-$(shell uname -m).tar.gz$(RESET)"

# Docker build (if you want to add Docker support later)
.PHONY: docker
docker: ## Build Docker image
	@echo "$(BLUE)🐳 Building Docker image...$(RESET)"
	@docker build -t $(BINARY_NAME):$(VERSION) .
	@echo "$(GREEN)✅ Docker image built: $(BINARY_NAME):$(VERSION)$(RESET)"

# Watch for changes and rebuild (requires cargo-watch)
.PHONY: watch
watch: ## Watch for changes and rebuild automatically
	@echo "$(BLUE)👀 Watching for changes...$(RESET)"
	@command -v cargo-watch >/dev/null 2>&1 || { echo "$(YELLOW)⚠️  Installing cargo-watch...$(RESET)"; cargo install cargo-watch; }
	@cargo watch -x "build"

# Initialize git hooks (if using git)
.PHONY: git-hooks
git-hooks: ## Set up git hooks for development
	@echo "$(BLUE)🔗 Setting up git hooks...$(RESET)"
	@echo '#!/bin/bash\nmake fmt-check lint' > .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "$(GREEN)✅ Git hooks configured$(RESET)"

# Show project info
.PHONY: info
info: ## Show project information
	@echo "$(CYAN)Project: $(BINARY_NAME)$(RESET)"
	@echo "$(CYAN)Version: $(VERSION)$(RESET)"
	@echo "$(CYAN)Rust version: $$(rustc --version)$(RESET)"
	@echo "$(CYAN)Cargo version: $$(cargo --version)$(RESET)"
	@echo "$(CYAN)Build directory: $(BUILD_DIR)$(RESET)"
	@echo "$(CYAN)Install path: $(INSTALL_PATH)$(RESET)"

# All targets that don't create files
.PHONY: help check-deps clean check fmt fmt-check lint test test-coverage debug build build-optimized install uninstall run run-args bench docs audit update quality dev release package docker watch git-hooks info
# Test targets
.PHONY: test-unit test-integration test-all test-coverage test-performance
test-unit: ## Run unit tests only
	@echo "$(BLUE)🧪 Running unit tests...$(RESET)"
	@cargo test --lib --tests unit

test-integration: ## Run integration tests only
	@echo "$(BLUE)🧪 Running integration tests...$(RESET)"
	@cargo test --tests integration

test-performance: ## Run performance tests
	@echo "$(BLUE)⚡ Running performance tests...$(RESET)"
	@cargo test --tests integration::test_performance --release

test-all: ## Run all tests (unit + integration)
	@echo "$(BLUE)🧪 Running all tests...$(RESET)"
	@cargo test --all-targets

test-watch: ## Watch tests and re-run on changes
	@echo "$(BLUE)👀 Watching tests...$(RESET)"
	@command -v cargo-watch >/dev/null 2>&1 || { echo "$(YELLOW)⚠️  Installing cargo-watch...$(RESET)"; cargo install cargo-watch; }
	@cargo watch -x "test --all-targets"

test-coverage-html: ## Generate HTML coverage report
	@echo "$(BLUE)📊 Generating HTML coverage report...$(RESET)"
	@command -v cargo-tarpaulin >/dev/null 2>&1 || { echo "$(YELLOW)⚠️  Installing cargo-tarpaulin...$(RESET)"; cargo install cargo-tarpaulin; }
	@cargo tarpaulin --out Html --output-dir coverage --all-targets
	@echo "$(GREEN)✅ Coverage report generated: coverage/tarpaulin-report.html$(RESET)"

# Override the default test target to be more comprehensive
test: test-all ## Run all tests (comprehensive)
