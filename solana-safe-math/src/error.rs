use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SafeMathError {
    #[error("Arithmetic operation resulted in a boundary overflow")]
    ArithmeticOverflow,
    #[error("Integer could not be safely casted to the target type")]
    ConversionFailed,
}
