//! Error types for Converter

/// It is used to represent different types of errors that can occur during the conversion process.
/// Each variant of the enum represents a specific type of error,
/// and it can contain additional data associated with the error,
/// such as error messages or other relevant information.
#[derive(Debug, thiserror::Error)]
pub enum ConvertError {
    /// Failed to write section config target.
    #[error(
        "Path was interpreted as the path to ActorBase, but the ID directory is missing. expected: [..]/DynamicAnimationReplacer/{{ESP name}}/{{ID Number}}, actual: {0}"
    )]
    MissingBaseId(String),

    /// Failed to write section config target.
    #[error("Failed to write section config target: {0}")]
    FailedWriteSectionConfig(String),

    /// Never converted.
    #[error("Never converted.")]
    NeverConverted,

    /// No such paths exist.
    #[error("No such paths exist: \"{0}\"")]
    NonExistPath(String),

    /// Nothing in the specified path.
    #[error("Nothing in the specified path")]
    NotFoundEntry,
    /// Could not find files with ".mohidden" extension.
    #[error("Could not find files with \".mohidden\" extension")]
    NotFoundUnhideTarget,
    /// Not found "DynamicAnimationReplacer" directory.
    #[error("Not found \"DynamicAnimationReplacer\" directory")]
    NotFoundDarDir,
    /// Not found file name.
    #[error("Not found file name")]
    NotFoundFileName,
    /// Not found "OpenAnimationReplacer" directory.
    #[error("Not found \"OpenAnimationReplacer\" directory")]
    NotFoundOarDir,
    /// Not found DAR priority(Number) directory.
    #[error("Not found DAR priority(Number) directory")]
    NotFoundPriorityDir,

    /// Incomplete conversion.
    #[error("Incomplete conversion")]
    IncompleteConversion,
    /// Incomplete parse DAR. Remain:
    #[error("Incomplete parse DAR. Remain:\n{0}")]
    IncompleteParseDar(String),

    /// DAR syntax error.
    #[error("DAR syntax error.:\n{0}")]
    InvalidDarSyntax(String),

    /// This is not valid utf8.
    #[error("This is not valid utf8")]
    InvalidUtf8,

    /// Condition error.
    #[error(transparent)]
    ConditionError(#[from] crate::conditions::ConditionError),

    /// Parse error.
    #[error(transparent)]
    ParseError(#[from] crate::condition_parser::ParseError),

    /// Convert json error.
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    /// Parse integer error.
    #[error(transparent)]
    ParseIntError(#[from] core::num::ParseIntError),

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    /// Thread join error.
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
}

//? Implemented to facilitate testing with the `assert_eq!` macro.
impl PartialEq for ConvertError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::FailedWriteSectionConfig(l0), Self::FailedWriteSectionConfig(r0))
            | (Self::InvalidDarSyntax(l0), Self::InvalidDarSyntax(r0)) => l0 == r0,
            (Self::ConditionError(l0), Self::ConditionError(r0)) => l0 == r0,
            (Self::ParseError(l0), Self::ParseError(r0)) => l0 == r0,
            (Self::JsonError(l0), Self::JsonError(r0)) => l0.to_string() == r0.to_string(),
            (Self::ParseIntError(l0), Self::ParseIntError(r0)) => l0 == r0,
            (Self::IOError(l0), Self::IOError(r0)) => l0.kind() == r0.kind(),
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
