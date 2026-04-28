//! Error types for Converter

use snafu::Snafu;
use std::path::PathBuf;

/// Represents different types of errors that can occur during the conversion process.
#[derive(Debug, Snafu)]
pub enum ConvertError {
    /// Only `And` or `Or` can be converted to Vec.
    CastError,

    /// Path interpreted as the path to `ActorBase`, but the ID directory is missing.
    #[snafu(display("Path was interpreted as the path to ActorBase, but the ID directory is missing. expected: [..]/DynamicAnimationReplacer/{{ESP name}}/{{ID Number}}, actual: {}", path.display()))]
    MissingBaseId {
        /// path
        path: PathBuf,
    },

    /// Never converted.
    NeverConverted,

    /// No such paths exist.
    #[snafu(display("No such paths exist: \"{}\"", path.display()))]
    NonExistPath {
        /// path
        path: PathBuf,
    },

    /// Nothing in the specified path.
    NotFoundEntry,

    /// Could not find files with ".mohidden" extension.
    NotFoundUnhideTarget,

    /// Not found `DynamicAnimationReplacer` directory.
    NotFoundDarDir,

    /// Not found file name.
    NotFoundFileName,

    /// Not found `OpenAnimationReplacer` directory.
    NotFoundOarDir,

    /// Not found DAR priority (Number) directory.
    NotFoundPriorityDir,

    /// This is not valid UTF-8.
    InvalidUtf8,

    /// DAR syntax error with path.
    #[snafu(display("[DAR Syntax Error] {}\n{}", path.display(), source))]
    InvalidDarSyntax {
        /// path
        path: PathBuf,
        /// transparent
        source: dar_syntax::ReadableError,
    },

    /// OAR condition error(For `dar.try_into`)
    #[snafu(transparent)]
    ConditionError {
        /// transparent
        source: oar_conditions::error::Error,
    },

    /// JSON conversion error.
    #[snafu(transparent)]
    JsonError {
        /// transparent
        source: serde_json::Error,
    },

    /// Parse integer error.
    #[snafu(transparent)]
    ParseIntError {
        /// transparent
        source: core::num::ParseIntError,
    },

    /// I/O error.
    #[snafu(transparent)]
    IOError {
        /// transparent
        source: std::io::Error,
    },

    /// Async walkdir error.
    #[snafu(transparent)]
    AsyncWalkDirError {
        /// transparent
        source: async_walkdir::Error,
    },

    /// Thread join error.
    #[snafu(transparent)]
    JoinError {
        /// transparent
        source: tokio::task::JoinError,
    },

    #[snafu(display("Error reading mapping table from {}:\n{}", path.display(), source))]
    MappingTableError {
        path: PathBuf,
        source: mapping_table::reader::ReadableError,
    },

    #[allow(clippy::use_self)]
    #[snafu(display("Errors: \n{}", errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(",\n\n")))]
    NestedError { errors: Vec<ConvertError> },
}

// Implemented to facilitate testing with the `assert_eq!` macro.
impl PartialEq for ConvertError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::JsonError { source: l0 }, Self::JsonError { source: r0 }) => {
                l0.to_string() == r0.to_string()
            }
            (Self::ParseIntError { source: l0 }, Self::ParseIntError { source: r0 }) => l0 == r0,
            (Self::IOError { source: l0 }, Self::IOError { source: r0 }) => l0.kind() == r0.kind(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

/// A specialized [Result] type for the conversion process.
///
/// It is a shorthand for [`core::result::Result`] where the error type is defaulted
/// to [`ConvertError`]. This allows functions and methods in the conversion process
/// to conveniently use this type without explicitly specifying the error type.
///
/// # Examples
///
/// ```
/// use dar2oar_core::error::Result;
///
/// fn convert_something() -> Result<()> {
///     // Some conversion logic here
///     // ...
///
///     Ok(())
/// }
/// ```
pub type Result<T, Error = ConvertError> = core::result::Result<T, Error>;
