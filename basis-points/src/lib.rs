use bytemuck::{Pod, Zeroable};
#[cfg(feature = "decimal")]
use rust_decimal::Decimal;

use crate::error::BasisPointsError;
mod error;

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
pub struct BasisPoints(u16);

impl BasisPoints {
    pub const MAX: u16 = 10000;

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
}
