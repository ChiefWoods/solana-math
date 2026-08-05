# wrapped-decimal

A fixed-size wrapper around `rust_decimal::Decimal` for stable byte-layout use cases.

## What it provides

- `WrappedDecimal([u8; 16])` with `#[repr(C)]`
- POD-safe representation for account/state layouts
- `From<Decimal>` and `From<WrappedDecimal>` conversions
- Optional Anchor derives via feature flags

## Install

```bash
cargo add wrapped-decimal
```

## Features

- `anchor`: enable Anchor serialization/IDL traits

## Example

```rust
use rust_decimal::Decimal;
use wrapped_decimal::WrappedDecimal;

let d = Decimal::new(12345, 2); // 123.45
let wrapped = WrappedDecimal::from(d);
let back = Decimal::from(wrapped);
assert_eq!(back, d);
```
