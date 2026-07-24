# Updating changelogs (agent guide)

This workspace uses [`cargo-changeset`](https://github.com/lukidoescode/cargo-changeset) to version crates and update per-crate `CHANGELOG.md` files (shipped on crates.io).

| Surface | Location | Who updates it |
|---------|----------|----------------|
| Release intent | `.changeset/` | Add with `just changeset` in the same change as the work |
| Crate changelog | `<crate>/CHANGELOG.md` | Written by `just version` from pending changesets |

Do **not** hand-edit version headers in `CHANGELOG.md` or bump `Cargo.toml` versions for a release — use the commands below.

## Commands

```bash
just changeset                          # record intent (interactive)
just changeset -p basis-points -b patch -c fixed -m "Fix …"
just changeset-status                   # preview bumps
just version                            # apply bumps + changelog entries
just version --dry-run                  # preview only
just publish basis-points               # crates.io (after commit)
```

Typical flow (same idea as JS Changesets):

1. Make code changes
2. `just changeset` (or `cargo changeset verify` to see what's uncovered)
3. Commit code + changeset files together
4. When releasing: `just version`, review the diff, commit
5. `just publish <crate>` in dependency order (`basis-points` → `wrapped-decimal`; skip `solana-safe-math`)
6. `just github-release <crate>` (or the **GitHub Release** workflow) to tag and post notes on the Releases page

## When to add a changeset

Add a changeset for any **user-facing** change to a published crate:

- New API, feature flag, or type
- Behavior change or bug fix users might notice
- **Breaking change** (use bump `major`, or `minor` under 0.x with `effective-minor` semantics)

Skip for internal-only work (CI, comment-only, no API impact) unless you intend a release note. Use bump `none` for documented internal changes that should not force a version bump (promoted to `patch` on release by default).

## Writing the description

The changeset body becomes the changelog entry. Write it for crate consumers:

- Good: `Add optional \`codama\` feature for \`BasisPoints\`.`
- Bad: `Refactored src/lib.rs`

Categories: `added`, `changed`, `deprecated`, `removed`, `fixed`, `security` (default `changed`).

## Checklist for agents

- [ ] Ran `cargo changeset verify` (or added changesets for every uncovered crate)
- [ ] Changeset description is user-facing, not an implementation diary
- [ ] Did not manually bump `Cargo.toml` / `CHANGELOG.md` version headers
- [ ] Did not publish unless the user asked
