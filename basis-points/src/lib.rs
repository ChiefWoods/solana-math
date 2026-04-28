//! `basis-points` provides a validated [`BasisPoints`] newtype for percentage-like
//! values represented in basis points (`0..=10_000`).
//!
//! The crate includes integer conversions and optional
//! [`rust_decimal::Decimal`](https://docs.rs/rust_decimal/latest/rust_decimal/struct.Decimal.html)
//! conversions behind the `decimal` feature.

use bytemuck::{Pod, Zeroable};
#[cfg(feature = "decimal")]
use rust_decimal::Decimal;

use crate::error::BasisPointsError;
pub mod error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Zeroable, Pod)]
#[cfg_attr(
    feature = "anchor",
    derive(
        anchor_lang::InitSpace,
        anchor_lang::prelude::AnchorSerialize,
        anchor_lang::prelude::AnchorDeserialize
    )
)]
#[repr(C)]
/// A validated basis-points value in the inclusive range `0..=10_000`.
pub struct BasisPoints(u16);

impl BasisPoints {
    /// The maximum valid basis-points value (`10000`, equal to 100%).
    pub const MAX: u16 = 10000;

    /// Creates a new [`BasisPoints`] after validating the input range.
    ///
    /// Returns [`BasisPointsError::InvalidBasisPoints`] if `bps > 10_000`.
    pub fn new(bps: u16) -> Result<Self, BasisPointsError> {
        if bps > Self::MAX {
            return Err(BasisPointsError::InvalidBasisPoints);
        }

        Ok(Self(bps))
    }
}

impl From<BasisPoints> for u16 {
    fn from(bps: BasisPoints) -> Self {
        bps.0
    }
}

impl From<BasisPoints> for u32 {
    fn from(bps: BasisPoints) -> Self {
        u32::from(bps.0)
    }
}

impl From<BasisPoints> for u64 {
    fn from(bps: BasisPoints) -> Self {
        u64::from(bps.0)
    }
}

impl From<BasisPoints> for u128 {
    fn from(bps: BasisPoints) -> Self {
        u128::from(bps.0)
    }
}

#[cfg(feature = "decimal")]
impl From<BasisPoints> for Decimal {
    fn from(bps: BasisPoints) -> Self {
        Decimal::from(bps.0)
    }
}

#[cfg(feature = "decimal")]
impl TryFrom<Decimal> for BasisPoints {
    type Error = BasisPointsError;

    fn try_from(decimal: Decimal) -> Result<Self, Self::Error> {
        let bps = decimal
            .try_into()
            .map_err(|_| BasisPointsError::ConversionFailed)?;
        BasisPoints::new(bps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(BasisPoints::new(10000), Ok(BasisPoints(10000)));
    }

    #[test]
    fn test_new_error() {
        assert_eq!(
            BasisPoints::new(10001),
            Err(BasisPointsError::InvalidBasisPoints)
        );
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_from_basis_points_to_decimal() {
        let bps = BasisPoints::new(1234).unwrap();
        let decimal: Decimal = bps.into();

        assert_eq!(decimal, Decimal::new(1234, 0));
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_try_from_decimal_to_basis_points() {
        let decimal = Decimal::new(4321, 0);
        let bps = BasisPoints::try_from(decimal);

        assert_eq!(bps, Ok(BasisPoints(4321)));
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_round_trip_basis_points_decimal_basis_points() {
        let original = BasisPoints::new(9999).unwrap();
        let decimal: Decimal = original.into();
        let round_trip = BasisPoints::try_from(decimal);

        assert_eq!(round_trip, Ok(original));
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_try_from_decimal_fractional_truncates() {
        let decimal = Decimal::new(12345, 1); // 1234.5
        let bps = BasisPoints::try_from(decimal);

        assert_eq!(bps, Ok(BasisPoints(1234)));
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_try_from_decimal_above_basis_points_max_error() {
        let decimal = Decimal::new(10001, 0);
        let bps = BasisPoints::try_from(decimal);

        assert_eq!(bps, Err(BasisPointsError::InvalidBasisPoints));
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_try_from_decimal_u16_overflow_error() {
        let decimal = Decimal::new(70000, 0);
        let bps = BasisPoints::try_from(decimal);

        assert_eq!(bps, Err(BasisPointsError::ConversionFailed));
    }
}
