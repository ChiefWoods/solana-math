# solana-math

Checked math and numeric conversion traits that return explicit errors instead of panicking.

## What it provides

- `SafeMath`: checked `add/sub/mul/div`
- `SafeMathAssign`: checked `*_assign` helpers
- `SafeConvert`: checked integer conversions
- `SafeMathError` with `Overflow`, `Underflow`, and `ConversionFailed`

## Install

```bash
cargo add solana-math
```

## Features

Numeric trait impls are gated by features:

- Unsigned: `u8`, `u16`, `u32`, `u64`, `u128`
- Signed: `i8`, `i16`, `i32`, `i64`, `i128`
- `decimal`: enable `rust_decimal::Decimal` support
- `basis-points`: enable `basis_points::BasisPoints` support
- `brine-fp`: enable `brine_fp::{UnsignedNumeric, SignedNumeric}` support
- `anchor`: convert `SafeMathError` into Anchor `ProgramError` / `Error`
- `quasar`: convert `SafeMathError` into Quasar `ProgramError`, and enable `zeropod` POD integer safe math

## Example

```rust
use solana_math::SafeMath;

let x: u64 = 10;
assert_eq!(x.safe_add(5)?, 15);
# Ok::<(), solana_math::SafeMathError>(())
```
