//! Error types for `solana-safe-math`.
//!
//! This module defines [`SafeMathError`], used by checked arithmetic and safe
//! numeric conversion helpers.

use thiserror::Error;

/// Errors returned by safe arithmetic operations and numeric conversions.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SafeMathError {
    /// Arithmetic operation exceeded the maximum numeric bound of the target type.
    #[error("Arithmetic operation resulted in a boundary overflow")]
    Overflow,
    /// Arithmetic operation went below the minimum numeric bound of the target type.
    #[error("Arithmetic operation resulted in a boundary underflow")]
    Underflow,
    /// Integer could not be safely cast to the target numeric type.
    #[error("Integer could not be safely casted to the target type")]
    ConversionFailed,
}
