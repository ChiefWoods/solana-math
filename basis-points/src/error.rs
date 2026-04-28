use thiserror::Error;

/// Errors returned by `BasisPoints` validation and conversions.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum BasisPointsError {
    /// Basis points must be less than or equal to `10_000`.
    #[error("Basis points must be less than or equal to 10000")]
    InvalidBasisPoints,
    /// Decimal could not be safely cast to the target integer type.
    #[error("Decimal could not be safely casted to the target type")]
    ConversionFailed,
}
