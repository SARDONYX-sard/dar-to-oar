//! DAR parser errors
pub mod readable_error;

/// DAR error
#[derive(Debug, Clone, snafu::Snafu, PartialEq, Eq)]
#[snafu(visibility(pub))]
pub enum DarError {
    // New error variant to represent an empty collection.
    /// Failed to pop from an empty collection
    EmptyCollectionError,

    /// Readable parser position error
    #[snafu(transparent)]
    ReadableError {
        /// transparent
        source: readable_error::ReadableError,
    },
}

/// DAR Result
pub type Result<T, Error = DarError> = core::result::Result<T, Error>;
