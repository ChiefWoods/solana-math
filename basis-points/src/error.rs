use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BasisPointsError {
    #[error("Basis points must be less than or equal to 10000")]
    InvalidBasisPoints,
    #[error("Decimal could not be safely casted to the target type")]
    ConversionFailed,
}
