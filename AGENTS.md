# Agent guide — solana-math

Instructions for AI agents working in this repository.

## Project

Rust workspace of small, dependency-light math crates for Solana programs and off-chain tooling. Each crate is published independently to [crates.io](https://crates.io).

| Crate | Path | Role |
|-------|------|------|
| `basis-points` | `basis-points/` | Validated `BasisPoints` type (`0..=10_000`) |
| `wrapped-decimal` | `wrapped-decimal/` | POD-safe 16-byte `Decimal` wrapper |
| `solana-safe-math` | `solana-safe-math/` | Checked arithmetic traits (no panics) |

Workspace dependency order: `basis-points` → `wrapped-decimal` → `solana-safe-math`.

## Principles

1. **Minimal scope** — Change only what the task requires. Match existing style in the crate you touch.
2. **Safe math** — Prefer explicit errors (`Result`, `thiserror`) over panics in library code.
3. **Determinism** — Avoid behavior that differs between on-chain and off-chain unless behind a feature flag.
4. **Feature flags** — Optional integrations stay optional (`anchor`, `codama`, `decimal`, integer width flags in `solana-safe-math`). Do not enable them in `default` features.
5. **Documentation** — Public items get `///` docs consistent with sibling crates. Crate roots have a short module-level `//!` overview.
6. **No drive-by edits** — Do not reformat, rename, or refactor unrelated code.

## Commands

Use [just](https://github.com/casey/just) when available; otherwise use the equivalent `cargo` commands.

```bash
just check          # cargo check --workspace --all-features --locked
just test           # cargo test --workspace --all-features --locked
just clippy         # deny warnings
just fmt            # format (CI uses --check)
just changeset      # add a changeset (needs cargo-changeset)
just version        # bump versions + changelogs from pending changesets
cargo fmt --all -- --check   # what CI / pre-commit runs
```

CI (`.github/workflows/tests.yml`) runs: check → fmt check → clippy → docs → test, all with `--all-features --locked`.

Local Git hooks (`.husky/pre-commit`) run check, clippy, and fmt check. Set `NO_HUSKY_HOOKS=1` to skip hooks in automation.

## Making changes

### Which crate to edit

- Type / conversion for basis points → `basis-points`
- Fixed-size decimal storage → `wrapped-decimal`
- Generic checked ops or shared traits → `solana-safe-math`

If a change spans crates, include each affected crate in the changeset (see below).

### Code style

- Run `just fmt` before finishing.
- `clippy` must pass with `-D warnings`.
- Keep `Cargo.lock` in sync (`--locked` in CI); run `cargo build` / `cargo test` after dependency changes.
- New public API surface: add or extend tests in the same crate.

### Changesets and changelogs

Versioning and per-crate `CHANGELOG.md` updates are managed by [`cargo-changeset`](https://github.com/lukidoescode/cargo-changeset).

**Full guide:** [docs/updating-changelogs.md](docs/updating-changelogs.md)

Quick rules:

- User-facing crate changes need a changeset (`just changeset` / `cargo changeset add`).
- Do not hand-bump `Cargo.toml` versions or invent changelog version headers — run `just version` at release time.
- Run `cargo changeset verify` before committing when the working tree has crate source changes.

## Releases

Publishing is **not** done by agents unless the user explicitly asks. There is no Publish CI workflow.

```bash
just version                 # apply pending changesets
# commit the version bump
just publish basis-points    # then wrapped-decimal; skip solana-safe-math
just github-release basis-points   # tag + GitHub Releases page (from CHANGELOG.md)
```

Release order: `basis-points` → `wrapped-decimal` → `solana-safe-math` (`publish = false` on crates.io; still use `just github-release` for GitHub).

Optional tags use the form `crate-name@vX.Y.Z` (e.g. `basis-points@v0.3.2`). The **GitHub Release** workflow (`.github/workflows/github-release.yml`) can create the same release via `workflow_dispatch`.

## Git

- Do **not** create commits, tags, or pushes unless the user asks.
- Do **not** publish crates unless the user asks.
- Do **not** add markdown files the user did not request (except when this guide or linked docs are the task).

## Key paths

```
basis-points/          # crate + CHANGELOG.md
wrapped-decimal/
solana-safe-math/
.changeset/            # pending changeset files
.github/workflows/     # tests.yml, github-release.yml
docs/updating-changelogs.md
Justfile               # workspace + changeset tasks
```

## Checklist before handing off

- [ ] Changes limited to the relevant crate(s)
- [ ] `just check`, `just clippy`, `just test` pass (or equivalent)
- [ ] `cargo fmt --all -- --check` passes
- [ ] User-facing changes covered by a changeset (`cargo changeset verify`)
- [ ] No version bumps or publish steps unless requested
