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
    cargo clean --doc
    cargo doc --workspace --no-deps --all-features --open

# --- cargo-changeset (install: cargo install cargo-changeset) ---
# Flow: just changeset → just version → commit → just release <crate>
# `just release` pushes crate@vX.Y.Z; the Release workflow publishes + GitHub release.

# Add a changeset (interactive, or pass flags: -p NAME -b patch -m "…").
changeset *args:
    cargo changeset add {{args}}

# Show pending changesets and projected bumps.
changeset-status:
    cargo changeset status

# Verify changed crates have changeset coverage (optional: --base main).
changeset-verify *args:
    cargo changeset verify {{args}}

# Apply pending changesets: bump Cargo.toml + CHANGELOG.md (no commit/tag).
# Preview with: just version --dry-run
version *args:
    cargo changeset release {{args}}

# Tag current Cargo.toml version and push (triggers Release workflow).
# Usage: `just release basis-points` (order: basis-points → wrapped-decimal → solana-math)
release crate:
    #!/usr/bin/env bash
    set -euo pipefail
    crate="{{crate}}"
    manifest="crates/${crate}/Cargo.toml"
    name=$(grep -m1 '^name = ' "$manifest" | sed 's/.*"\(.*\)".*/\1/')
    version=$(grep -m1 '^version = ' "$manifest" | sed 's/.*"\(.*\)".*/\1/')
    tag="${name}@v${version}"

    if git rev-parse -q --verify "refs/tags/${tag}" >/dev/null; then
      echo "error: tag ${tag} already exists" >&2
      exit 1
    fi

    git tag -a "$tag" -m "Publish ${name} v${version}"
    git push origin "refs/tags/${tag}"
    echo "Pushed ${tag}; Release workflow will publish to crates.io and create the GitHub release."
