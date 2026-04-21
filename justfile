# lucid-lint — task runner
# Install just: https://github.com/casey/just
# Run `just` to see available recipes.

set shell := ["bash", "-uc"]
set dotenv-load := true

# Default: list recipes
default:
    @just --list

# First-time setup: install components, hooks, and sanity check
setup:
    @echo "Installing Rust components..."
    rustup component add rustfmt clippy llvm-tools-preview
    @echo "Installing cargo tools..."
    cargo install --locked cargo-insta cargo-llvm-cov cargo-dist mdbook || true
    @echo "Installing pre-commit hooks..."
    command -v pre-commit >/dev/null && pre-commit install || echo "pre-commit not found; skipping hook install"
    @echo "Running sanity check..."
    @just check-quick
    @echo "Setup complete."

# Fast feedback loop: format, lint, test
check-quick: fmt-check lint test

# Full quality gate: format, lint, test, coverage, docs build
check: fmt-check lint test coverage-summary docs-build

# Run all tests
test:
    cargo test --all-features --workspace

# Re-run tests on file change (requires cargo-watch)
test-watch:
    cargo watch -x 'test --all-features --workspace'

# Run a specific test
test-one name:
    cargo test --all-features --workspace {{name}}

# Format code
fmt:
    cargo fmt --all

# Check formatting without applying
fmt-check:
    cargo fmt --all -- --check

# Run clippy with project deny-list
lint:
    cargo clippy --all-features --all-targets --workspace -- -D warnings

# Fix clippy warnings where possible
lint-fix:
    cargo clippy --all-features --all-targets --workspace --fix --allow-dirty --allow-staged -- -D warnings

# Update insta snapshots after intentional output changes
snapshot:
    cargo insta test --all-features --review

# Accept all pending snapshot changes
snapshot-accept:
    cargo insta accept

# Coverage: full report to lcov.info
coverage:
    cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# Coverage: summary only
coverage-summary:
    cargo llvm-cov --all-features --workspace --summary-only

# Coverage: HTML report at target/llvm-cov/html/
coverage-html:
    cargo llvm-cov --all-features --workspace --html
    @echo "Report at target/llvm-cov/html/index.html"

# Mirror ROADMAP.md into docs/src/roadmap.md (rewrites relative links)
sync-roadmap:
    python3 scripts/sync-roadmap.py

# Build the mdBook documentation
docs-build: sync-roadmap
    cd docs && mdbook build
    python3 scripts/sanitize-stock-css.py

# Serve docs locally with hot reload
docs-serve: sync-roadmap
    cd docs && mdbook serve --open

# Pre-deploy gate: verify the built book doesn't ship banned stock fonts.
# Not wired into `just check` (mdbook build is too slow for every dev loop);
# intended for release-candidate branches and the CI docs-publish workflow.
docs-check-clean: docs-build
    #!/usr/bin/env bash
    set -euo pipefail
    # Match active font declarations only — `font-family: "Banned"`, CSS
    # custom-property values like `--mono-font: "Banned"`, and @font-face
    # sources like `src: local('Banned')`. License attributions and our
    # own override-documentation comments are ignored.
    active='(font-family|--[a-z-]+-font|local)[^;]*[\x27"](Open Sans|Source Code Pro)\b'
    hits=$(grep -RIEn --include='*.css' --include='*.html' "$active" docs/book/ || true)
    if [ -n "$hits" ]; then
      echo "docs-check-clean: banned font reference(s) found:" >&2
      echo "$hits" >&2
      exit 1
    fi
    echo "docs-check-clean: clean — no banned font references in docs/book/"

# Clean generated artifacts
clean:
    cargo clean
    rm -rf docs/book
    rm -f lcov.info

# Dogfood: run lucid-lint on its own documentation (dev-doc profile for technical docs)
dogfood:
    cargo run --release -- check --profile dev-doc README.md RULES.md ROADMAP.md CHANGELOG.md CONTRIBUTING.md CODING_STANDARDS.md AGENTS.md docs/src

# Release dry-run using cargo-dist
release-check:
    cargo dist plan

# Build a release binary for the local target
release-build:
    cargo build --release

# Publish to crates.io (requires CARGO_REGISTRY_TOKEN)
publish:
    cargo publish --locked

# Install the binary from source
install:
    cargo install --path . --locked
