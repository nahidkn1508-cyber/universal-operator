use super::error::OperatorError;

pub type OperatorResult<T> = Result<T, OperatorError>;
