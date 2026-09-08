.PHONY: help
help: ## Show this help.
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "%-10s : %s\n", $$1, $$2}'

.PHONY: check 
check: ## Code format and lint check.
	cargo fmt --check
	cargo clippy -- -D warnings

.PHONY: test-all
test-all: ## Run all tests.
	cargo test --verbose -- --nocapture

.PHONY: clean-all
clean-all: ## Clean up all packages.
	cargo clean
