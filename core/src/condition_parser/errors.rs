//! Dar to OAR cast error definitions
use crate::values::ValueError;

/// Couldn't parse in DAR to OAR processing Errors
#[derive(Debug, Clone, snafu::Snafu, PartialEq, Eq)]
pub enum ParseError {
    /// Expected {expected}. but got {actual}
    UnexpectedValue {
        /// Expected value
        expected: String,
        /// Actual value
        actual: String,
    },

    /// OAR condition error
    #[snafu(transparent)]
    ConditionError {
        /// transparent
        source: crate::conditions::ConditionError,
    },

    /// Value error.
    #[snafu(transparent)]
    ValueError {
        /// transparent
        source: ValueError,
    },

    /// DAR error.
    #[snafu(transparent)]
    DarError {
        /// transparent
        source: crate::dar_syntax::errors::DarError,
    },
}

/// Condition parser Error.
pub(super) type Result<T, Error = ParseError> = core::result::Result<T, Error>;
