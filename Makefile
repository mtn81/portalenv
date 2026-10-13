.PHONY: help
help: ## Show this help.
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "%-10s : %s\n", $$1, $$2}'

.PHONY: lint
lint: ## Code format, lint and unused dependency check.
	cargo fmt --check
	cargo clippy -- -D warnings
	cargo machete

# `cargo machete --fix` exits with 1 even when it removed the unused dependencies
.PHONY: lint-fix
lint-fix: ## Auto-fix formatting, clippy lints and unused dependencies.
	cargo machete --fix || true
	cargo fmt
	cargo clippy --fix --allow-dirty --allow-staged -- -D warnings

.PHONY: test-all
test-all: ## Run all tests.
	cargo test --verbose -- --nocapture
	cd packages/tests/default_dotenv && cargo test --verbose -- --nocapture
	cd packages/tests/include_with_dotenv && cargo test --verbose -- --nocapture

.PHONY: udeps-all
udeps-all: ## Find unused dependencies in all packages (requires nightly and cargo-udeps).
	cargo +nightly udeps --workspace --all-targets
	cd packages/tests/default_dotenv && cargo +nightly udeps --all-targets
	cd packages/tests/include_with_dotenv && cargo +nightly udeps --all-targets

.PHONY: clean-all
clean-all: ## Clean up all packages.
	cargo clean
