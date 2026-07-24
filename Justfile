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
