//! `basis-points` provides a validated [`BasisPoints`] newtype for percentage-like
//! values represented in basis points (`0..=10_000`).
//!
//! The crate includes integer conversions and optional
//! [`rust_decimal::Decimal`](https://docs.rs/rust_decimal/latest/rust_decimal/struct.Decimal.html)
//! conversions behind the `decimal` feature. With `decimal` enabled,
//! [`BasisPoints`] converts to [`Decimal`] as a unit fraction in `0..=1`
//! (for example, `2500` bps becomes `0.25`).

#![cfg_attr(not(any(test, feature = "std")), no_std)]

use bytemuck::{Pod, Zeroable};
#[cfg(feature = "codama")]
use codama::CodamaType;
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
#[cfg_attr(feature = "codama", derive(CodamaType))]
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

    /// Checked addition. Returns `None` if the result exceeds `10_000`.
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        let lhs = u16::from(self);
        let rhs = u16::from(rhs);
        let result = lhs.checked_add(rhs)?;
        Self::new(result).ok()
    }

    /// Checked subtraction. Returns `None` if underflow occurs.
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        let lhs = u16::from(self);
        let rhs = u16::from(rhs);
        let result = lhs.checked_sub(rhs)?;
        Self::new(result).ok()
    }

    /// Checked multiplication. Returns `None` if the result exceeds `10_000`.
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        let lhs = u16::from(self);
        let rhs = u16::from(rhs);
        let result = lhs.checked_mul(rhs)?;
        Self::new(result).ok()
    }

    /// Checked division. Returns `None` on division-by-zero.
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        let lhs = u16::from(self);
        let rhs = u16::from(rhs);
        let result = lhs.checked_div(rhs)?;
        Self::new(result).ok()
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
    /// Converts basis points to a proportional rate in `0..=1` (divide by `10_000`).
    fn from(bps: BasisPoints) -> Self {
        Decimal::from(bps.0) / Decimal::from(BasisPoints::MAX)
    }
}

#[cfg(feature = "decimal")]
impl TryFrom<Decimal> for BasisPoints {
    type Error = BasisPointsError;

    /// Converts a proportional rate in `0..=1` to basis points (multiply by `10_000`).
    ///
    /// Fractional results truncate toward zero when casting to `u16`.
    fn try_from(decimal: Decimal) -> Result<Self, Self::Error> {
        let bps = (decimal * Decimal::from(BasisPoints::MAX))
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

    #[test]
    fn test_checked_add() {
        let lhs = BasisPoints::new(2500).unwrap();
        let rhs = BasisPoints::new(2500).unwrap();

        assert_eq!(lhs.checked_add(rhs), BasisPoints::new(5000).ok());
    }

    #[test]
    fn test_checked_add_overflow() {
        let lhs = BasisPoints::new(9000).unwrap();
        let rhs = BasisPoints::new(2000).unwrap();

        assert_eq!(lhs.checked_add(rhs), None);
    }

    #[test]
    fn test_checked_sub() {
        let lhs = BasisPoints::new(2500).unwrap();
        let rhs = BasisPoints::new(500).unwrap();

        assert_eq!(lhs.checked_sub(rhs), BasisPoints::new(2000).ok());
    }

    #[test]
    fn test_checked_sub_underflow() {
        let lhs = BasisPoints::new(500).unwrap();
        let rhs = BasisPoints::new(2500).unwrap();

        assert_eq!(lhs.checked_sub(rhs), None);
    }

    #[test]
    fn test_checked_mul() {
        let lhs = BasisPoints::new(100).unwrap();
        let rhs = BasisPoints::new(50).unwrap();

        assert_eq!(lhs.checked_mul(rhs), BasisPoints::new(5000).ok());
    }

    #[test]
    fn test_checked_mul_overflow() {
        let lhs = BasisPoints::new(200).unwrap();
        let rhs = BasisPoints::new(100).unwrap();

        assert_eq!(lhs.checked_mul(rhs), None);
    }

    #[test]
    fn test_checked_div() {
        let lhs = BasisPoints::new(5000).unwrap();
        let rhs = BasisPoints::new(100).unwrap();

        assert_eq!(lhs.checked_div(rhs), BasisPoints::new(50).ok());
    }

    #[test]
    fn test_checked_div_by_zero() {
        let lhs = BasisPoints::new(5000).unwrap();
        let rhs = BasisPoints::new(0).unwrap();

        assert_eq!(lhs.checked_div(rhs), None);
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_from_basis_points_to_decimal() {
        let bps = BasisPoints::new(1234).unwrap();
        let decimal: Decimal = bps.into();

        assert_eq!(decimal, Decimal::new(1234, 4));
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_from_basis_points_max_to_decimal_one() {
        let bps = BasisPoints::new(BasisPoints::MAX).unwrap();
        let decimal: Decimal = bps.into();

        assert_eq!(decimal, Decimal::ONE);
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_try_from_decimal_to_basis_points() {
        let decimal = Decimal::new(4321, 4);
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
        let decimal = Decimal::new(12345, 5); // 0.12345
        let bps = BasisPoints::try_from(decimal);

        assert_eq!(bps, Ok(BasisPoints(1234)));
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_try_from_decimal_above_basis_points_max_error() {
        let decimal = Decimal::new(10001, 4); // 1.0001
        let bps = BasisPoints::try_from(decimal);

        assert_eq!(bps, Err(BasisPointsError::InvalidBasisPoints));
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_try_from_decimal_u16_overflow_error() {
        let decimal = Decimal::new(7, 0); // 700% when scaled
        let bps = BasisPoints::try_from(decimal);

        assert_eq!(bps, Err(BasisPointsError::ConversionFailed));
    }
}
