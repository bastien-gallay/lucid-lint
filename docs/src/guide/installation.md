# Installation

## Via Cargo (recommended)

Once published to crates.io:

```bash
cargo install lucid-lint
```

This installs the `lucid-lint` binary into your Cargo bin directory, usually `~/.cargo/bin/`.

## From source

```bash
git clone https://github.com/bastien-gallay/lucid-lint
cd lucid-lint
cargo install --path .
```

## Pre-built binaries

Each release ships pre-built binaries for:

- Linux (`x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`)
- macOS (`aarch64-apple-darwin`, `x86_64-apple-darwin`)
- Windows (`x86_64-pc-windows-msvc`)

Download from the [GitHub releases page](https://github.com/bastien-gallay/lucid-lint/releases).

## Verify the installation

```bash
lucid-lint --version
```

## System requirements

- Rust 1.75 or newer (only needed if building from source)
- No runtime dependencies
