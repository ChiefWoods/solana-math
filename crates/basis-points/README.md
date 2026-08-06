# basis-points

A small, validated basis-points type for Solana-oriented Rust code.

## What it provides

- `BasisPoints` newtype with enforced range `0..=10_000`
- Conversions to integer types (`u16`, `u32`, `u64`, `u128`)
- Optional `rust_decimal` conversions (`BasisPoints` ↔ proportional rate in `0..=1`)
- Optional Anchor derives via feature flags

## Install

```bash
cargo add basis-points
```

## Features

- `decimal`: enable `rust_decimal` conversions
- `anchor`: enable Anchor serialization/IDL traits and convert `BasisPointsError` into Anchor `ProgramError`
- `quasar`: enable Quasar IDL build support and convert `BasisPointsError` into Quasar `ProgramError`
- `codama`: enable `CodamaType` derive support

## Example

```rust
use basis_points::BasisPoints;

let fee = BasisPoints::new(25)?; // 0.25%
let raw: u16 = fee.into();
assert_eq!(raw, 25);
# Ok::<(), basis_points::BasisPointsError>(())
```
