# jankurai-tools-dedup root command surface.
# One-command setup and validation lanes for agents and CI.
# Every lane below is deterministic, hermetic, and runnable from the repo root.

# Default: list available lanes.
default:
    @just --list

# One-command bootstrap: install the toolchain components this repo needs.
setup:
    rustup component add rustfmt clippy
    cargo fetch --locked

# Alias for setup so `just install` and `just bootstrap` also resolve.
install: setup

bootstrap: setup

# Deterministic fast lane: the narrowest proof loop for agent iteration.
# Narrow, per-package targets keep the iteration loop tight; nextest caches and
# parallelizes the run.
fast:
    cargo check -p jankurai-audit-dedup --locked
    cargo nextest run -p jankurai-audit-dedup
    cargo check --workspace --locked
    cargo nextest run --workspace

# Run the full local check: format, lint, fast lane, security, and audit.
check: fmt lint fast security audit

# Verify is an alias of check for agents that look for a `verify` lane.
verify: check

fmt:
    cargo fmt --all --check

lint:
    cargo clippy --workspace --all-targets --locked -- -D warnings

# Run the workspace test suite.
test:
    cargo test --workspace --locked

# Public-API drift check: the crate is a library, so a public-API break is a
# semver event. cargo public-api diffs the rustdoc surface against the baseline.
api-drift:
    cargo public-api --package jankurai-audit-dedup diff
    cargo semver-checks check-release --package jankurai-audit-dedup

# Security lane: secret scanning plus dependency vulnerability scanning.
# gitleaks scans for committed secrets; cargo audit checks the Rust dependency tree.
# The full supply-chain posture (SBOM, provenance, workflow lint, npm audit) runs
# through the canonical wrapper tools/security-lane.sh.
security:
    gitleaks detect --source . --no-banner --redact
    cargo audit
    bash tools/security-lane.sh

# Jankurai self-audit lane: writes the repo-score artifacts that CI uploads.
audit:
    bash ops/ci/audit.sh

# Print the declared version.
versions:
    cat VERSION
