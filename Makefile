.PHONY: setup test coverage coverage-html

# One-shot local development setup:
# - Ensures llvm-tools are available (for coverage)
# - Installs cargo-llvm-cov if missing
# - Builds the project once to fetch dependencies
setup:
	@echo "==> Setting up local development environment"
	@command -v rustup >/dev/null 2>&1 && rustup component add llvm-tools-preview || echo "rustup not found; skipping llvm-tools-preview component"
	@command -v cargo-llvm-cov >/dev/null 2>&1 || ( \
		if command -v brew >/dev/null 2>&1; then \
			echo "cargo-llvm-cov not found; attempting to install via Homebrew..."; \
			brew install cargo-llvm-cov || echo "Warning: Homebrew install of cargo-llvm-cov failed."; \
		else \
			echo "brew not found; skipping Homebrew install of cargo-llvm-cov."; \
		fi; \
		if ! command -v cargo-llvm-cov >/dev/null 2>&1; then \
			echo "cargo-llvm-cov still not found; attempting to install from crates.io (requires a recent Rust toolchain)..."; \
			cargo install cargo-llvm-cov || echo "Warning: cargo-llvm-cov install from crates.io failed; coverage commands will not be available."; \
		fi \
	)
	@cargo build

test:
	cargo test

# Run test coverage and show a summary in the terminal
coverage:
	cargo llvm-cov

# Generate an HTML coverage report and open it (macOS)
coverage-html:
	cargo llvm-cov --html
	open target/llvm-cov/html/index.html


