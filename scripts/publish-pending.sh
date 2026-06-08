#!/usr/bin/env bash
# Publish every crate with a pending CHANGELOG entry, in dependency order.
#
# Requires bash (do not run with `sh`).
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

pending="$("./scripts/detect-pending-releases.sh")"
if [[ "$pending" == "[]" ]]; then
  echo "No pending releases."
  exit 0
fi

create_release="${CREATE_RELEASE:-true}"
export AUTO_STASH=1

resolve_cliff_range() {
  local name=$1 old_tag=$2 new_tag=$3

  git fetch origin --tags --force

  if git rev-parse -q --verify "refs/tags/${old_tag}" >/dev/null; then
    echo "${old_tag}..${new_tag}"
    return 0
  fi

  local prev_tag
  prev_tag=$(git tag -l "${name}@v*" --sort=-v:refname | grep -Fv "${new_tag}" | head -1 || true)
  if [[ -n "$prev_tag" ]] && git rev-parse -q --verify "refs/tags/${prev_tag}" >/dev/null; then
    echo "Previous tag ${old_tag} not found; using ${prev_tag}..${new_tag}" >&2
    echo "${prev_tag}..${new_tag}"
    return 0
  fi

  echo "No previous tag found for ${name}; using ${new_tag}" >&2
  echo "${new_tag}"
}

write_changelog_release_notes() {
  local changelog=$1 version=$2 outfile=$3

  {
    echo "## What's new"
    echo ""
    awk -v ver="$version" '
      $0 == "## " ver { found=1; next }
      found && /^## / { exit }
      found { print }
    ' "$changelog"
  } >"$outfile"
}

create_github_release() {
  local crate=$1 name=$2 version=$3 old_tag=$4 new_tag=$5

  if [[ "$create_release" != "true" ]]; then
    return 0
  fi

  if ! git rev-parse -q --verify "refs/tags/${new_tag}" >/dev/null; then
    echo "Tag ${new_tag} not found; skipping GitHub release." >&2
    return 0
  fi

  if gh release view "$new_tag" >/dev/null 2>&1; then
    echo "GitHub release ${new_tag} already exists; skipping."
    return 0
  fi

  local cliff_range notes_written=false
  cliff_range=$(resolve_cliff_range "$name" "$old_tag" "$new_tag")
  if git cliff "$cliff_range" \
    --config .github/cliff.toml \
    --include-path "${crate}/**" \
    -o RELEASE_NOTES.md 2>/dev/null && [[ -s RELEASE_NOTES.md ]]; then
    notes_written=true
  fi

  if [[ "$notes_written" != "true" ]]; then
    echo "git-cliff failed for ${new_tag}; using CHANGELOG.md." >&2
    write_changelog_release_notes "${crate}/CHANGELOG.md" "$version" RELEASE_NOTES.md
  fi

  gh release create "$new_tag" --title "$new_tag" --notes-file RELEASE_NOTES.md
}

count=$(echo "$pending" | jq 'length')
i=0
while [[ "$i" -lt "$count" ]]; do
  item=$(echo "$pending" | jq -c ".[$i]")
  crate=$(echo "$item" | jq -r '.crate')
  version=$(echo "$item" | jq -r '.version')
  name=$(echo "$item" | jq -r '.name')

  previous=$(grep -m1 '^version = ' "${crate}/Cargo.toml" | sed 's/.*"\(.*\)".*/\1/')
  old_tag="${name}@v${previous}"
  new_tag="${name}@v${version}"

  echo "Publishing ${name} ${previous} -> ${version}"
  VERSION="$version" "./scripts/publish.sh" "$crate" version
  create_github_release "$crate" "$name" "$version" "$old_tag" "$new_tag"

  i=$((i + 1))
done
