#!/usr/bin/env bash
# Bump version, tag, push, and publish one workspace crate.
#
# Usage:
#   ./scripts/publish.sh <crate-dir> <level> [--dry-run]
#
# Examples:
#   ./scripts/publish.sh basis-points patch
#   ./scripts/publish.sh wrapped-decimal version --dry-run   # requires VERSION=0.2.0
set -euo pipefail

crate="${1:?crate directory required (e.g. basis-points)}"
level="${2:?release level required (patch|minor|major|version)}"
dry_run=false

for arg in "${@:3}"; do
  case "$arg" in
    --dry-run) dry_run=true ;;
    *) echo "Unknown argument: $arg" >&2; exit 1 ;;
  esac
done

if [[ "$level" == "version" ]]; then
  level="${VERSION:?set VERSION when using level \"version\"}"
fi

manifest="${crate}/Cargo.toml"
if [[ ! -f "$manifest" ]]; then
  echo "Manifest not found: $manifest" >&2
  exit 1
fi

name=$(grep -m1 '^name = ' "$manifest" | sed 's/.*"\(.*\)".*/\1/')
previous=$(grep -m1 '^version = ' "$manifest" | sed 's/.*"\(.*\)".*/\1/')

# workflow_run checkouts use a detached HEAD; cargo-release rejects that by default.
if [[ "$(git rev-parse --abbrev-ref HEAD)" == "HEAD" ]]; then
  git checkout -B main
fi

working_tree_clean() {
  git diff --quiet && git diff --cached --quiet && [[ -z "$(git ls-files --others --exclude-standard)" ]]
}

restore_release_stash() {
  if [[ "${PUBLISH_STASHED:-}" == "1" ]]; then
    git stash pop
    unset PUBLISH_STASHED
  fi
}

maybe_stash_for_release() {
  if working_tree_clean; then
    return 0
  fi

  if [[ "${AUTO_STASH:-}" != "1" ]]; then
    echo "error: uncommitted changes detected; commit or stash them, or set AUTO_STASH=1" >&2
    git status --short >&2
    exit 1
  fi

  echo "Stashing uncommitted changes before release..." >&2
  git stash push --include-untracked -m "publish: pre-release stash"
  export PUBLISH_STASHED=1
  trap restore_release_stash EXIT
}

maybe_stash_for_release

if $dry_run; then
  cargo release "$level" \
    --manifest-path "$manifest" \
    --dry-run
  restore_release_stash
  trap - EXIT
  exit 0
fi

cargo release "$level" \
  --manifest-path "$manifest" \
  --tag-name "${name}@v{{version}}" \
  --no-publish \
  --no-confirm \
  --execute

new_version=$(grep -m1 '^version = ' "$manifest" | sed 's/.*"\(.*\)".*/\1/')

cargo publish -p "$name" --locked

restore_release_stash
trap - EXIT

if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
  {
    echo "new_git_tag=${name}@v${new_version}"
    echo "old_git_tag=${name}@v${previous}"
    echo "crate_name=${name}"
    echo "crate_version=${new_version}"
  } >> "$GITHUB_OUTPUT"
fi
