#!/usr/bin/env bash
# List crates whose top CHANGELOG version is ahead of Cargo.toml.
#
# Prints a JSON array to stdout, in dependency order:
#   [{"crate":"basis-points","name":"basis-points","version":"0.3.2"}, ...]
set -euo pipefail

crates=(basis-points wrapped-decimal solana-safe-math)
entries=()

for crate in "${crates[@]}"; do
  manifest="${crate}/Cargo.toml"
  changelog="${crate}/CHANGELOG.md"

  if [[ ! -f "$manifest" || ! -f "$changelog" ]]; then
    echo "Missing manifest or changelog for ${crate}" >&2
    exit 1
  fi

  cargo_version=$(grep -m1 '^version = ' "$manifest" | sed 's/.*"\(.*\)".*/\1/')
  changelog_version=$(grep -m1 '^## ' "$changelog" | sed 's/^## //')
  name=$(grep -m1 '^name = ' "$manifest" | sed 's/.*"\(.*\)".*/\1/')

  if [[ "$changelog_version" == "$cargo_version" ]]; then
    continue
  fi

  if [[ "$(printf '%s\n%s\n' "$cargo_version" "$changelog_version" | sort -V | tail -1)" != "$changelog_version" ]]; then
    echo "CHANGELOG version ${changelog_version} is not ahead of Cargo.toml version ${cargo_version} for ${crate}" >&2
    exit 1
  fi

  entries+=("{\"crate\":\"${crate}\",\"name\":\"${name}\",\"version\":\"${changelog_version}\"}")
done

if ((${#entries[@]} == 0)); then
  printf '[]\n'
else
  (IFS=,; printf '[%s]\n' "${entries[*]}")
fi
