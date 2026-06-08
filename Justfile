# Rust workspace tasks. Run `just` or `just --list` to see recipes.

default:
    @just --list

workspace_flags := "--workspace --all-features --locked"

build:
    cargo build {{workspace_flags}}

test:
    cargo test {{workspace_flags}}

check:
    cargo check {{workspace_flags}}

clippy:
    cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

# Format all Rust sources (use `cargo fmt --all -- --check` in CI / pre-commit).
fmt:
    cargo fmt --all

doc:
    cargo doc --workspace --no-deps --all-features --open

# Publish one crate to crates.io only (no version bump or GitHub release).
# Prefer `just release` or the Publish GitHub Actions workflow for the full pipeline.
publish crate:
    cargo publish -p {{crate}} --locked

# Preview the next release (no changes committed).
# Usage: `just release-dry-run basis-points patch`
release-dry-run crate level:
    ./scripts/publish.sh {{crate}} {{level}} --dry-run

# Bump version, tag, push, and publish one crate locally.
# Usage: `just release basis-points patch`
# Exact version: `VERSION=0.4.0 just release basis-points version`
# Full-release order: basis-points → wrapped-decimal → solana-safe-math
release crate level:
    ./scripts/publish.sh {{crate}} {{level}}

# Publish every crate whose CHANGELOG version is ahead of Cargo.toml.
release-pending:
    ./scripts/publish-pending.sh

# Generate GitHub-style release notes for a crate between two tags.
# Usage: `just release-notes basis-points basis-points@v0.3.0 basis-points@v0.3.1`
release-notes crate from_tag to_tag:
    git cliff {{from_tag}}..{{to_tag}} --config .github/cliff.toml --include-path "{{crate}}/**"

# Tag current crate versions (run once before the first automated GitHub release).
bootstrap-tags push="":
    #!/usr/bin/env bash
    set -euo pipefail
    if [ "{{push}}" = "push" ]; then
      ./scripts/bootstrap-tags.sh --push
    else
      ./scripts/bootstrap-tags.sh
    fi
