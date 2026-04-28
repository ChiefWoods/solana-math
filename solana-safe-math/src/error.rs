use thiserror::Error;

/// Errors returned by safe arithmetic operations and numeric conversions.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SafeMathError {
    /// Arithmetic operation exceeded the numeric bounds of the target type.
    #[error("Arithmetic operation resulted in a boundary overflow")]
    ArithmeticOverflow,
    /// Integer could not be safely cast to the target numeric type.
    #[error("Integer could not be safely casted to the target type")]
    ConversionFailed,
}
