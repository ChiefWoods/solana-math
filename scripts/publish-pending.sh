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

resolve_cliff_range() {
  local name=$1 old_tag=$2 new_tag=$3

  git fetch origin --tags --force

  if git rev-parse -q --verify "refs/tags/${old_tag}" >/dev/null; then
    echo "${old_tag}..${new_tag}"
    return
  fi

  local prev_tag
  prev_tag=$(git tag -l "${name}@v*" --sort=-v:refname | grep -Fv "${new_tag}" | head -1 || true)
  if [[ -n "$prev_tag" ]] && git rev-parse -q --verify "refs/tags/${prev_tag}" >/dev/null; then
    echo "Previous tag ${old_tag} not found; using ${prev_tag}..${new_tag}" >&2
    echo "${prev_tag}..${new_tag}"
    return
  fi

  echo "No previous tag found for ${name}; using ${new_tag}" >&2
  echo "${new_tag}"
}

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
    cliff_range=$(resolve_cliff_range "$name" "$old_tag" "$new_tag")
    git cliff "$cliff_range" \
      --config .github/cliff.toml \
      --include-path "${crate}/**" \
      -o RELEASE_NOTES.md
    gh release create "$new_tag" --title "$new_tag" --notes-file RELEASE_NOTES.md
  fi
done
