//! Represents an error that can occur while working with conditions.

/// Represents an error that can occur while working with conditions.
#[derive(Debug, Clone, snafu::Snafu, PartialEq, Eq)]
pub enum Error {
    /// Only `And` or `Or` can be converted to Vec.
    CastError,

    /// Expected {expected}. but got {actual}
    UnexpectedValue {
        /// Expected value
        expected: String,
        /// Actual value
        actual: String,
    },

    #[allow(clippy::use_self)]
    #[snafu(display("{}", errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")))]
    NestedError { errors: Vec<Error> },
}

#[cfg(test)]
pub(crate) type Result<T, Error = serde_json::Error> = core::result::Result<T, Error>;
