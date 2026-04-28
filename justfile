# lucid-lint — task runner
# Install just: https://github.com/casey/just
# Run `just` to see available recipes.

set shell := ["bash", "-uc"]
set dotenv-load := true

# Default: list recipes
default:
    @just --list

# ── Setup ────────────────────────────────────────────────

# First-time setup: install components, hooks, and sanity check
[group('setup')]
setup:
    @echo "Installing Rust components..."
    rustup component add rustfmt clippy llvm-tools-preview
    @echo "Installing cargo tools..."
    cargo install --locked cargo-insta cargo-llvm-cov cargo-dist mdbook agnix-cli || true
    @echo "Installing pre-commit hooks..."
    command -v pre-commit >/dev/null && pre-commit install || echo "pre-commit not found; skipping hook install"
    @echo "Running sanity check..."
    @just check-quick
    @echo "Setup complete."

# ── Top-level quality gates ──────────────────────────────

# Fast feedback loop: format, lint, test
[group('check')]
check-quick: fmt-check lint test

# Full quality gate: format, lint, test, coverage, docs build
[group('check')]
check: fmt-check lint test coverage-summary docs-build lint-agents

# Validate AGENTS.md / .agent/ / CLAUDE.md against agnix rules.
# Requires `cargo install agnix-cli`. Config: .agnix.toml.
[group('check')]
lint-agents:
    @command -v agnix >/dev/null || { echo "agnix not installed; run: cargo install agnix-cli"; exit 1; }
    agnix --strict .

# ── Tests ────────────────────────────────────────────────

# Run all tests
[group('test')]
test:
    cargo test --all-features --workspace

# Re-run tests on file change (requires cargo-watch)
[group('test')]
test-watch:
    cargo watch -x 'test --all-features --workspace'

# Run a specific test
[group('test')]
test-one name:
    cargo test --all-features --workspace {{name}}

# Run the parser/engine hot-path micro-benchmarks (criterion).
[group('test')]
bench:
    cargo bench --bench parser_hotpath

# Run cargo-mutants on a single file (default: structure/sentence_too_long).
# Override with: just mutants src/scoring.rs
# Surviving mutants point at missing tests, not bugs to fix in this run.
[group('test')]
mutants file="src/rules/structure/sentence_too_long.rs":
    cargo mutants --file {{file}} --timeout 60 --no-shuffle

# ── Format ───────────────────────────────────────────────

# Format code
[group('fmt')]
fmt:
    cargo fmt --all

# Check formatting without applying
[group('fmt')]
fmt-check:
    cargo fmt --all -- --check

# ── Lint ─────────────────────────────────────────────────

# Run clippy with project deny-list
[group('lint')]
lint:
    cargo clippy --all-features --all-targets --workspace -- -D warnings

# Fix clippy warnings where possible
[group('lint')]
lint-fix:
    cargo clippy --all-features --all-targets --workspace --fix --allow-dirty --allow-staged -- -D warnings

# Dogfood: run lucid-lint on its own documentation (dev-doc profile for technical docs).
# Score-only gate (F80): warnings are informational, --min-score is the CI signal.
# Floor sits below the current baseline so prose drift trips it; ratchet up as we fix.
#
# Scope is authored prose only. RULES.md, ROADMAP.md and CHANGELOG.md are
# project-state databases (dense table cells, Keep-a-Changelog narrative
# rows) where line-length-wide and excessive-commas fire on structure rather
# than prose — keep them out of the dogfood gate.
[group('lint')]
dogfood:
    cargo run --release -- check --profile dev-doc --no-fail-on-warning --min-score 60 README.md CONTRIBUTING.md CODING_STANDARDS.md AGENTS.md docs/src

# ── Snapshots ────────────────────────────────────────────

# Update insta snapshots after intentional output changes
[group('snapshot')]
snapshot:
    cargo insta test --all-features --review

# Accept all pending snapshot changes
[group('snapshot')]
snapshot-accept:
    cargo insta accept

# ── Coverage ─────────────────────────────────────────────

# Coverage: full report to lcov.info
[group('coverage')]
coverage:
    cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# Coverage: summary only
[group('coverage')]
coverage-summary:
    cargo llvm-cov --all-features --workspace --summary-only

# Coverage: HTML report at target/llvm-cov/html/
[group('coverage')]
coverage-html:
    cargo llvm-cov --all-features --workspace --html
    @echo "Report at target/llvm-cov/html/index.html"

# ── Docs ─────────────────────────────────────────────────

# Mirror ROADMAP.md into docs/src/roadmap.md (rewrites relative links)
[group('docs')]
sync-roadmap:
    python3 scripts/sync-roadmap.py

# Build the mdBook documentation
[group('docs')]
docs-build: sync-roadmap
    cd docs && mdbook build
    python3 scripts/sanitize-stock-css.py
    python3 scripts/sync_lang_counterparts.py

# EN pages without FR counterparts are informational only — the
# translation backlog is tracked as F25, not enforced here.
# Validate that every FR doc page has an EN counterpart (F92). CI gate.
[group('docs')]
docs-lang-check: docs-build
    python3 scripts/sync_lang_counterparts.py --check

# Serve docs locally with hot reload.
#
# Pinned to port 3010 instead of mdbook's default 3000 — the latter
# collides with Node dev servers, Next.js, Rails, React toolchains,
# and the VS Code Live Preview extension. 3010 is far enough away to
# stay clear of the common dev-port band (3000-3001).
# `MDBOOK_OUTPUT__HTML__SITE_URL=/` overrides `book.toml`'s
# `site-url = "/lucid-lint/"` for local serve only. Production builds
# on GitHub Pages still use the `/lucid-lint/` prefix; this override
# only affects what ends up in `<base href>` on 404.html, so stylesheet
# and script URLs resolve correctly when `mdbook serve` answers
# unknown paths with the 404 template.
#
# Serve docs locally with hot reload (port 3010).
[group('docs')]
docs-serve: sync-roadmap
    cd docs && MDBOOK_OUTPUT__HTML__SITE_URL=/ mdbook serve --open --port 3010

# Not wired into `just check` (mdbook build is too slow for every dev loop);
# intended for release-candidate branches and the CI docs-publish workflow.
# Pre-deploy gate: verify the built book doesn't ship banned stock fonts.
[group('docs')]
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

# Skips shared.tape — it is a preset sourced by the other tapes.
# Render every VHS tape under docs/tapes/ (requires `vhs` on PATH).
[group('docs')]
tapes:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! command -v vhs >/dev/null; then
        echo "vhs not found — install via 'brew install vhs' or 'go install github.com/charmbracelet/vhs@latest'" >&2
        exit 1
    fi
    mkdir -p docs/src/assets/tty
    for tape in docs/tapes/*.tape; do
        name=$(basename "$tape")
        if [ "$name" = "shared.tape" ]; then continue; fi
        echo "rendering $tape"
        vhs "$tape"
    done

# ── Text fixtures ────────────────────────────────────────

# Fetch + clean + convert examples/texts.yaml into Markdown fixtures (dev-only).
[group('texts')]
texts: texts-fetch texts-clean texts-convert

# Fetch raw sources listed in examples/texts.yaml.
[group('texts')]
texts-fetch:
    uv run scripts/texts_fetch.py

# Clean fetched sources (strip boilerplate, normalize).
[group('texts')]
texts-clean:
    uv run scripts/texts_clean.py

# Convert cleaned sources to Markdown fixtures.
[group('texts')]
texts-convert:
    uv run scripts/texts_convert.py

# Routing plan, no network I/O — useful for reviewing the YAML changes
[group('texts')]
texts-plan:
    uv run scripts/texts_fetch.py --dry-run

# Regenerate the coverage table in examples/texts.md from texts.yaml
[group('texts')]
texts-coverage:
    uv run scripts/texts_coverage.py

# Fail if examples/texts.md drifts from texts.yaml
[group('texts')]
texts-coverage-check:
    uv run scripts/texts_coverage.py --check

# Unit tests for the coverage generator
[group('texts')]
texts-coverage-test:
    uv run scripts/test_texts_coverage.py

# ── Release ──────────────────────────────────────────────

# Release dry-run using cargo-dist
[group('release')]
release-check:
    cargo dist plan

# Build a release binary for the local target
[group('release')]
release-build:
    cargo build --release

# Publish to crates.io (requires CARGO_REGISTRY_TOKEN)
[group('release')]
publish:
    cargo publish --locked

# Install the binary from source
[group('release')]
install:
    cargo install --path . --locked

# ── Housekeeping ─────────────────────────────────────────

# Clean generated artifacts
[group('housekeeping')]
clean:
    cargo clean
    rm -rf docs/book
    rm -f lcov.info
