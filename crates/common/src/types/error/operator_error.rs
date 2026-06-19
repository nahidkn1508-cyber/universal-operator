//! Root error type for the Universal AI Operator.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorError {
    pub message: String,
}

impl fmt::Display for OperatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for OperatorError {}
