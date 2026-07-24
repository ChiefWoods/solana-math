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
# Flow: just changeset → just version → commit → just publish <crate> → just github-release <crate>

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

# Publish one crate to crates.io (after just version + commit). Not for solana-safe-math.
publish crate:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ "{{crate}}" = "solana-safe-math" ]; then
      echo "solana-safe-math is not published to crates.io (publish = false)." >&2
      exit 1
    fi
    cargo publish -p {{crate}} --locked

# Tag current Cargo.toml version and create a GitHub release from CHANGELOG.md (needs gh).
# Usage: `just github-release basis-points`
github-release crate:
    #!/usr/bin/env bash
    set -euo pipefail
    crate="{{crate}}"
    manifest="${crate}/Cargo.toml"
    changelog="${crate}/CHANGELOG.md"
    name=$(grep -m1 '^name = ' "$manifest" | sed 's/.*"\(.*\)".*/\1/')
    version=$(grep -m1 '^version = ' "$manifest" | sed 's/.*"\(.*\)".*/\1/')
    new_tag="${name}@v${version}"

    notes="$(mktemp)"
    trap 'rm -f "$notes"' EXIT
    {
      echo "## What's new"
      echo ""
      awk -v ver="$version" '
        $0 ~ "^## \\[" ver "\\]" { found=1; next }
        found && /^## / { exit }
        found { print }
      ' "$changelog"
    } >"$notes"

    if ! grep -q '[^[:space:]]' <(tail -n +3 "$notes"); then
      echo "error: no CHANGELOG section for version ${version} in ${changelog}" >&2
      exit 1
    fi

    if ! git rev-parse -q --verify "refs/tags/${new_tag}" >/dev/null; then
      git tag -a "$new_tag" -m "Publish ${name} v${version}"
      git push origin "refs/tags/${new_tag}"
    fi

    if gh release view "$new_tag" >/dev/null 2>&1; then
      echo "GitHub release ${new_tag} already exists; skipping."
      exit 0
    fi

    gh release create "$new_tag" --title "$new_tag" --notes-file "$notes"
