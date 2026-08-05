//! `wrapped-decimal` provides [`WrappedDecimal`], a fixed-size wrapper around
//! [`rust_decimal::Decimal`] with a stable 16-byte representation.
//!
//! This is useful for Solana account/state layouts that require predictable,
//! POD-safe binary encoding.

use bytemuck::{Pod, Zeroable};
#[cfg(feature = "codama")]
use codama::CodamaType;
use rust_decimal::Decimal;

#[derive(Clone, Copy, Zeroable, Pod, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "anchor",
    derive(
        anchor_lang::InitSpace,
        anchor_lang::prelude::AnchorSerialize,
        anchor_lang::prelude::AnchorDeserialize
    )
)]
#[cfg_attr(feature = "codama", derive(CodamaType))]
#[repr(C)]
/// A fixed-size, POD-safe wrapper around [`rust_decimal::Decimal`].
///
/// The decimal is stored as the 16-byte serialized representation from
/// `rust_decimal`, making this type suitable for account/state layouts that
/// require stable byte size.
pub struct WrappedDecimal([u8; 16]);

impl From<Decimal> for WrappedDecimal {
    /// Serializes a [`Decimal`] into its 16-byte wrapped representation.
    fn from(d: Decimal) -> Self {
        Self(d.serialize())
    }
}

impl From<WrappedDecimal> for Decimal {
    /// Deserializes the wrapped 16-byte representation back into a [`Decimal`].
    fn from(w: WrappedDecimal) -> Self {
        Decimal::deserialize(w.0)
    }
}
