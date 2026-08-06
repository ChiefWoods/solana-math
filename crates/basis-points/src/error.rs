//! Error types for `basis-points`.
//!
//! This module defines [`BasisPointsError`], used when validation fails or when
//! conversions cannot be performed safely.

use thiserror::Error;

/// Errors returned by `BasisPoints` validation and conversions.
///
/// Errors are numbered from the range 9100 - 9199 when used in conversions to custom program errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum BasisPointsError {
    /// Basis points must be less than or equal to `10_000`.
    #[error("Basis points must be less than or equal to 10000")]
    InvalidBasisPoints = 9100,
    /// Decimal could not be safely cast to the target integer type.
    #[error("Decimal could not be safely casted to the target type")]
    ConversionFailed = 9101,
}

// Anchor and Quasar both re-export the same `solana_program_error::ProgramError`
// type. Prefer the Quasar impl when both features are enabled so `--all-features`
// does not produce conflicting `From` implementations.
#[cfg(all(feature = "anchor", not(feature = "quasar")))]
impl From<BasisPointsError> for anchor_lang::prelude::ProgramError {
    fn from(e: BasisPointsError) -> Self {
        Self::Custom(e as u32)
    }
}

#[cfg(feature = "quasar")]
impl From<BasisPointsError> for quasar_lang::prelude::ProgramError {
    fn from(e: BasisPointsError) -> Self {
        Self::Custom(e as u32)
    }
}
