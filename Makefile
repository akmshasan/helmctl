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
	@awk 'BEGIN {FS = ":.*##"} /^[a-zA-Z_-]+:.*##/ { printf "  $(GREEN)%-20s$(RESET) %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

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
	@rm -rf coverage/
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

# =============================================================================
# TEST TARGETS WITH PROPER DEPENDENCIES
# =============================================================================

# Run unit tests
.PHONY: test-unit
test-unit: check-deps ## Run unit tests only
	@echo "$(BLUE)🧪 Running unit tests...$(RESET)"
	@cargo test --lib --tests unit
	@echo "$(GREEN)✅ Unit tests completed$(RESET)"

# Run integration tests
.PHONY: test-integration
test-integration: check-deps ## Run integration tests only
	@echo "$(BLUE)🧪 Running integration tests...$(RESET)"
	@cargo test --tests integration
	@echo "$(GREEN)✅ Integration tests completed$(RESET)"

# Run all tests
.PHONY: test-all
test-all: check-deps ## Run all tests (unit + integration)
	@echo "$(BLUE)🧪 Running all tests...$(RESET)"
	@cargo test --all-targets
	@echo "$(GREEN)✅ All tests completed$(RESET)"

# Run performance tests
.PHONY: test-performance
test-performance: check-deps ## Run performance tests
	@echo "$(BLUE)⚡ Running performance tests...$(RESET)"
	@cargo test --tests integration::test_performance --release
	@echo "$(GREEN)✅ Performance tests completed$(RESET)"

# =============================================================================
# COVERAGE TARGETS (DEPEND ON TESTS)
# =============================================================================

# Install coverage tool if needed
.PHONY: install-coverage-tool
install-coverage-tool:
	@echo "$(BLUE)📦 Checking coverage tool...$(RESET)"
	@command -v cargo-tarpaulin >/dev/null 2>&1 || { \
		echo "$(YELLOW)⚠️  Installing cargo-tarpaulin...$(RESET)"; \
		cargo install cargo-tarpaulin; \
	}
	@echo "$(GREEN)✅ Coverage tool ready$(RESET)"

# Generate basic coverage (depends on all tests passing)
.PHONY: test-coverage
test-coverage: test-all install-coverage-tool ## Generate coverage report (requires tests to pass)
	@echo "$(BLUE)📊 Generating coverage report...$(RESET)"
	@cargo tarpaulin --out Xml --output-dir coverage
	@echo "$(GREEN)✅ Coverage report generated: coverage/cobertura.xml$(RESET)"

# Generate HTML coverage report (depends on all tests passing)
.PHONY: test-coverage-html
test-coverage-html: test-all install-coverage-tool ## Generate HTML coverage report (requires tests to pass)
	@echo "$(BLUE)📊 Generating HTML coverage report...$(RESET)"
	@cargo tarpaulin --out Html --output-dir coverage --all-targets
	@echo "$(GREEN)✅ HTML coverage report generated: coverage/tarpaulin-report.html$(RESET)"
	@echo "$(CYAN)👀 Open with: open coverage/tarpaulin-report.html$(RESET)"

# Generate comprehensive coverage (depends on all tests passing)
.PHONY: test-coverage-all
test-coverage-all: test-all install-coverage-tool ## Generate all coverage formats (requires tests to pass)
	@echo "$(BLUE)📊 Generating comprehensive coverage reports...$(RESET)"
	@cargo tarpaulin --out Html --out Xml --out Lcov --output-dir coverage --all-targets
	@echo "$(GREEN)✅ All coverage reports generated in coverage/$(RESET)"

# =============================================================================
# WATCH TARGETS
# =============================================================================

# Install watch tool if needed
.PHONY: install-watch-tool
install-watch-tool:
	@echo "$(BLUE)📦 Checking watch tool...$(RESET)"
	@command -v cargo-watch >/dev/null 2>&1 || { \
		echo "$(YELLOW)⚠️  Installing cargo-watch...$(RESET)"; \
		cargo install cargo-watch; \
	}
	@echo "$(GREEN)✅ Watch tool ready$(RESET)"

# Watch for changes and run tests
.PHONY: test-watch
test-watch: install-watch-tool ## Watch for changes and re-run tests automatically
	@echo "$(BLUE)👀 Watching for changes...$(RESET)"
	@cargo watch -x "test --all-targets"

# Watch for changes and run tests with coverage
.PHONY: test-watch-coverage
test-watch-coverage: install-watch-tool install-coverage-tool ## Watch for changes and re-run tests with coverage
	@echo "$(BLUE)👀 Watching for changes with coverage...$(RESET)"
	@cargo watch -s "make test-coverage-html"

# =============================================================================
# BUILD TARGETS
# =============================================================================

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

# =============================================================================
# QUALITY TARGETS (WITH PROPER DEPENDENCIES)
# =============================================================================

# Full quality check pipeline (tests must pass before coverage)
.PHONY: quality
quality: fmt-check lint test-all ## Run full quality check pipeline
	@echo "$(GREEN)✅ All quality checks passed$(RESET)"

# Quality check with coverage (depends on quality passing)
.PHONY: quality-with-coverage
quality-with-coverage: quality test-coverage-html ## Run quality checks and generate coverage
	@echo "$(GREEN)✅ Quality checks and coverage completed$(RESET)"

# =============================================================================
# WORKFLOW TARGETS
# =============================================================================

# Development workflow
.PHONY: dev
dev: clean check test-all build ## Complete development workflow
	@echo "$(GREEN)✅ Development workflow completed$(RESET)"

# Release workflow (comprehensive)
.PHONY: release
release: clean quality-with-coverage build-optimized ## Complete release workflow with coverage
	@echo "$(GREEN)✅ Release workflow completed$(RESET)"
	@echo "$(CYAN)🎉 Ready for release!$(RESET)"

# CI workflow (what CI should run)
.PHONY: ci
ci: fmt-check lint test-all test-coverage build ## CI workflow with coverage
	@echo "$(GREEN)✅ CI workflow completed$(RESET)"

# =============================================================================
# INSTALLATION TARGETS
# =============================================================================

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

# =============================================================================
# UTILITY TARGETS
# =============================================================================

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

# Package for distribution
.PHONY: package
package: build-optimized ## Package binary for distribution
	@echo "$(BLUE)📦 Packaging for distribution...$(RESET)"
	@mkdir -p dist
	@cp $(RELEASE_DIR)/$(BINARY_NAME) dist/
	@cp README.md dist/
	@tar -czf dist/$(BINARY_NAME)-$(VERSION)-$(shell uname -s)-$(shell uname -m).tar.gz -C dist $(BINARY_NAME) README.md
	@echo "$(GREEN)✅ Package created: dist/$(BINARY_NAME)-$(VERSION)-$(shell uname -s)-$(shell uname -m).tar.gz$(RESET)"

# Show project info
.PHONY: info
info: ## Show project information
	@echo "$(CYAN)Project: $(BINARY_NAME)$(RESET)"
	@echo "$(CYAN)Version: $(VERSION)$(RESET)"
	@echo "$(CYAN)Rust version: $$(rustc --version)$(RESET)"
	@echo "$(CYAN)Cargo version: $$(cargo --version)$(RESET)"
	@echo "$(CYAN)Build directory: $(BUILD_DIR)$(RESET)"
	@echo "$(CYAN)Install path: $(INSTALL_PATH)$(RESET)"

# =============================================================================
# MAIN WORKFLOW TARGETS (ALIASES)
# =============================================================================

# Default test target (comprehensive)
.PHONY: test
test: test-all ## Run all tests (alias for test-all)

# Coverage target (depends on tests)
.PHONY: coverage
coverage: test-coverage-html ## Generate HTML coverage (depends on tests passing)

# All targets that don't create files
.PHONY: help check-deps clean check fmt fmt-check lint test-unit test-integration test-all test-performance test-coverage test-coverage-html test-coverage-all test-watch test-watch-coverage debug build build-optimized quality quality-with-coverage dev release ci install uninstall run run-args docs audit update package info test coverage install-coverage-tool install-watch-tool
