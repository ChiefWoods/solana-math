#!/usr/bin/env bash
# Create git tags for the current Cargo.toml versions (run once before the first
# automated GitHub release so git-cliff has a previous tag to diff against).
#
# Usage:
#   ./scripts/bootstrap-tags.sh          # create missing tags locally
#   ./scripts/bootstrap-tags.sh --push   # create missing tags and push them
set -euo pipefail

push=false
if [[ "${1:-}" == "--push" ]]; then
  push=true
fi

for crate in basis-points wrapped-decimal solana-safe-math; do
  manifest="${crate}/Cargo.toml"
  name=$(grep -m1 '^name = ' "$manifest" | sed 's/.*"\(.*\)".*/\1/')
  version=$(grep -m1 '^version = ' "$manifest" | sed 's/.*"\(.*\)".*/\1/')
  tag="${name}@v${version}"

  if git rev-parse -q --verify "refs/tags/${tag}" >/dev/null; then
    echo "exists: ${tag}"
  else
    git tag -a "${tag}" -m "Publish ${name} v${version}"
    echo "created: ${tag}"
  fi
done

if $push; then
  git push origin --tags
fi
