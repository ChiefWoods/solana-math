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

# --- cargo-changeset (install: cargo install cargo-changeset) ---
# Flow: just changeset → just version → commit

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
