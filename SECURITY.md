# Security Policy

## Supported Versions

`lucid-lint` is currently in pre-release (v0.1). Only the latest version receives security updates.

## Reporting a Vulnerability

If you discover a security vulnerability, **please do not open a public issue**.

Instead, use GitHub's [private vulnerability reporting](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing/privately-reporting-a-security-vulnerability) feature on this repository.

We will:

1. Acknowledge receipt within 48 hours.
2. Investigate and validate the issue.
3. Develop a fix and coordinate disclosure timing with you.
4. Credit you in the release notes unless you prefer to remain anonymous.

## Scope

Relevant security concerns for `lucid-lint`:

- **Parsing vulnerabilities** (panics, DoS via crafted input)
- **Arbitrary code execution** through configuration files
- **Path traversal** in file loading
- **Dependency vulnerabilities** when not caught by `cargo-audit`

Out of scope:

- False positives or false negatives in linting rules — those are regular bugs.
- Issues in dependencies that do not affect `lucid-lint`'s functionality.
