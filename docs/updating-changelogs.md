# Updating changelogs (agent guide)

This workspace maintains **two** release-note surfaces:

| Surface | Location | Who updates it |
|---------|----------|----------------|
| Crate changelog | `<crate>/CHANGELOG.md` | **Manual** — update in the same PR as the change |
| GitHub release | GitHub Releases (`crate-name@vX.Y.Z`) | **Local** — `git-cliff` when creating a release |

Agents must keep per-crate `CHANGELOG.md` files current.

## Which file to edit

Edit only the changelog for the crate whose public API or behavior changed:

- `basis-points/CHANGELOG.md`
- `wrapped-decimal/CHANGELOG.md`
- `solana-safe-math/CHANGELOG.md`

If a change spans multiple crates, update each affected changelog.

Each file is listed in that crate's `Cargo.toml` `include` array and ships on crates.io.

## When to update

Add or extend an entry when the PR includes any **user-facing** change:

- New API, feature flag, or type
- Behavior change or bug fix users might notice
- **Breaking change** (always call out explicitly)

Skip changelog updates for internal-only work (CI, refactors with no API impact, comment-only edits) unless the team expects a release note.

## Format

Follow the existing style in each crate's `CHANGELOG.md`:

```markdown
# Changelog

## 0.3.2
- Added optional `foo` feature for `Bar`.
- **Breaking:** `Baz::qux` now returns `Result` instead of panicking.

## 0.3.1
- Previous release entry.
```

Rules:

1. **Newest version first** — insert a new `## X.Y.Z` section directly under `# Changelog`.
2. **Version header** — must match the version in that crate's `Cargo.toml` **at release time**. During development, use the version the change will ship with (usually the next bump).
3. **Bullets** — one line per change; start with a past-tense verb (`Added`, `Fixed`, `Removed`, `Renamed`).
4. **Breaking changes** — prefix the bullet with `**Breaking:**`.
5. **Scope** — describe what changed for crate users, not implementation detail.

## Release workflow

Publishing is local only (`just release <crate> <level>`). CI runs tests; it does not publish.

Dependency order when releasing multiple crates in one cycle:

1. `basis-points`
2. `wrapped-decimal`
3. `solana-safe-math` (GitHub release only — not published to crates.io)

Before publishing, confirm:

- The crate's `CHANGELOG.md` has a section for the version being released.
- `Cargo.toml` `version` is still the **previous** release (the top changelog header is the target version); the release command bumps it to match.

At release time, `Cargo.toml` `version` must match the section header being shipped.

GitHub release notes are generated from git commits between tags via `git-cliff`. Crate `CHANGELOG.md` entries should still be written in plain language for docs.rs and crates.io readers.

## Checklist for agents

- [ ] Identified the correct `<crate>/CHANGELOG.md` file(s)
- [ ] Added a `## X.Y.Z` section at the top (or extended the unreleased top section if one exists)
- [ ] Marked breaking changes with `**Breaking:**`
- [ ] Described user-visible impact, not internal refactors
- [ ] Did not edit other crates' changelogs unless those crates changed
