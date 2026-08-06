# solana-math

A small family of Rust crates for safe and deterministic math utilities used in Solana programs and supporting libraries.

## Crates

| Crate | Version | Description |
|-------|---------|-------------|
| [`basis-points`](https://crates.io/crates/basis-points) | [![Crates.io](https://img.shields.io/crates/v/basis-points.svg)](https://crates.io/crates/basis-points) | Validated `BasisPoints` type (`0..=10_000`) with optional `rust_decimal` conversions. |
| [`solana-math`](https://crates.io/crates/solana-math) | [![Crates.io](https://img.shields.io/crates/v/solana-math.svg)](https://crates.io/crates/solana-math) | Checked arithmetic and conversion traits that return explicit errors instead of panicking. |
| [`wrapped-decimal`](https://crates.io/crates/wrapped-decimal) | [![Crates.io](https://img.shields.io/crates/v/wrapped-decimal.svg)](https://crates.io/crates/wrapped-decimal) | Fixed-size, POD-safe wrapper for `rust_decimal::Decimal` using a stable 16-byte representation. |

## Workspace commands

```bash
just check
just test
just doc
just changeset    # add release intent (requires cargo-changeset)
just version      # bump versions + changelogs from changesets
just release <crate>  # push crate@vX.Y.Z tag → Release workflow
```

Install the release helper once: `cargo install cargo-changeset`. Publishing is tag-driven via trusted publishing.

## Notes

- Optional Anchor derives are available behind the `anchor` feature where applicable.
