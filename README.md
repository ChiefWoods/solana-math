# solana-math

A small family of Rust crates for safe and deterministic math utilities used in Solana programs and supporting libraries.

## Crates

- `basis-points`: A validated `BasisPoints` type (`0..=10_000`) with optional `rust_decimal` conversions.
- `solana-math`: Checked arithmetic and conversion traits that return explicit errors instead of panicking.
- `wrapped-decimal`: A fixed-size, POD-safe wrapper for `rust_decimal::Decimal` using a stable 16-byte representation.

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
