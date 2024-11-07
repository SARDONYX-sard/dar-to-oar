//! Dar to OAR cast error definitions
use crate::values::ValueError;

/// Couldn't parse in DAR to OAR processing Errors
#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum ParseError {
    /// - 1st arg: Expected value
    /// - 2nd arg: Actual value
    #[error("Expected {0}. but got {1}")]
    UnexpectedValue(String, String),

    /// - 1st arg: Expected value
    /// - 2nd arg: Actual value
    #[error("{expected} number of arguments is required, but in fact {actual} retrieved.")]
    NotEnoughArguments {
        /// Expected value
        expected: usize,
        /// Actual value
        actual: usize,
    },

    /// Value error.
    #[error(transparent)]
    ValueError(#[from] ValueError),
}

/// Condition parser Error.
pub(super) type Result<T, Error = ParseError> = core::result::Result<T, Error>;
