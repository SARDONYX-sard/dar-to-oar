//! Represents an error that can occur while working with conditions.

/// Represents an error that can occur while working with conditions.
#[derive(Debug, Clone, snafu::Snafu, PartialEq, Eq)]
#[snafu(visibility(pub))]
pub enum ConditionError {
    /// Only `And` or `Or` can be converted to Vec.
    CastError,

    /// Expected {expected}. but got {actual}
    UnexpectedValue {
        /// Expected value
        expected: String,
        /// Actual value
        actual: String,
    },
}
