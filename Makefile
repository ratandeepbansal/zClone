.PHONY: check test run build fmt clippy clean help

help: ## Show this help message
	@echo "zClone - Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

check: ## Check if project compiles
	@echo "🔍 Checking project..."
	@cargo check

test: ## Run tests
	@echo "🧪 Running tests..."
	@cargo test

run: ## Run the application
	@echo "🚀 Running zClone..."
	@cargo run

build: ## Build release version
	@echo "🔨 Building release..."
	@cargo build --release

fmt: ## Format code
	@echo "✨ Formatting code..."
	@cargo fmt

clippy: ## Run clippy linter
	@echo "📎 Running clippy..."
	@cargo clippy --all-features

clean: ## Clean build artifacts
	@echo "🧹 Cleaning..."
	@cargo clean

verify: check clippy ## Verify Phase 1 is complete
	@echo "✅ Phase 1 verification complete!"

all: fmt clippy check test build ## Run all checks and build
	@echo "✅ All tasks complete!"
