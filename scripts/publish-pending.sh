#!/usr/bin/env bash
# Publish every crate with a pending CHANGELOG entry, in dependency order.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

pending="$("./scripts/detect-pending-releases.sh")"
if [[ "$pending" == "[]" ]]; then
  echo "No pending releases."
  exit 0
fi

create_release="${CREATE_RELEASE:-true}"

mapfile -t items < <(echo "$pending" | jq -c '.[]')

for item in "${items[@]}"; do
  crate=$(echo "$item" | jq -r '.crate')
  version=$(echo "$item" | jq -r '.version')
  name=$(echo "$item" | jq -r '.name')

  previous=$(grep -m1 '^version = ' "${crate}/Cargo.toml" | sed 's/.*"\(.*\)".*/\1/')
  old_tag="${name}@v${previous}"
  new_tag="${name}@v${version}"

  echo "Publishing ${name} ${previous} -> ${version}"
  VERSION="$version" "./scripts/publish.sh" "$crate" version

  if [[ "$create_release" == "true" ]]; then
    git cliff "${old_tag}..${new_tag}" \
      --config .github/cliff.toml \
      --include-path "${crate}/**" \
      -o RELEASE_NOTES.md
    gh release create "$new_tag" --title "$new_tag" --notes-file RELEASE_NOTES.md
  fi
done
