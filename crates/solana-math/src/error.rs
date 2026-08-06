//! Error types for `solana-math`.
//!
//! This module defines [`SafeMathError`], used by checked arithmetic and safe
//! numeric conversion helpers.

use thiserror::Error;

/// Errors returned by safe arithmetic operations and numeric conversions.
///
/// Errors are numbered from the range 9000 - 9999 when used in conversions to custom program errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SafeMathError {
    /// Arithmetic operation exceeded the maximum numeric bound of the target type.
    #[error("Arithmetic operation resulted in a boundary overflow")]
    Overflow = 9000,
    /// Arithmetic operation went below the minimum numeric bound of the target type.
    #[error("Arithmetic operation resulted in a boundary underflow")]
    Underflow = 9001,
    /// Integer could not be safely cast to the target numeric type.
    #[error("Integer could not be safely casted to the target type")]
    ConversionFailed = 9002,
}

// Anchor and Quasar both re-export the same `solana_program_error::ProgramError`
// type. Prefer the Quasar impl when both features are enabled so `--all-features`
// does not produce conflicting `From` implementations.
#[cfg(all(feature = "anchor", not(feature = "quasar")))]
impl From<SafeMathError> for anchor_lang::prelude::ProgramError {
    fn from(e: SafeMathError) -> Self {
        Self::Custom(e as u32)
    }
}

#[cfg(feature = "quasar")]
impl From<SafeMathError> for quasar_lang::prelude::ProgramError {
    fn from(e: SafeMathError) -> Self {
        Self::Custom(e as u32)
    }
}
