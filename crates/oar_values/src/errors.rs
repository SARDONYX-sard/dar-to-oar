//! Represents an error that can occur while working with value parse.

/// Represents an error that can occur while working with value parse.
#[derive(Debug, Clone, snafu::Snafu, PartialEq, Eq)]
pub enum ValueError {
    /// Expected {expected}, but got {actual}
    CastError {
        /// Expected value
        expected: String,
        /// Actual value
        actual: String,
    },
}
