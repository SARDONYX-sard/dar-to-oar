#[derive(Debug, thiserror::Error)]
pub enum ConvertError {
    #[error("Failed to write section config target: {0}")]
    FailedWriteSectionConfig(String),
    #[error("Neither 1st or 3rd person \"DynamicAnimationReplacer.mohidden\" found.")]
    NotFoundUnhideTarget,
    #[error("Not found \"OpenAnimationReplacer\" directory")]
    NotFoundOarDir,
    #[error("Not found \"DynamicAnimationReplacer\" directory")]
    NotFoundDarDir,
    #[error("Not found file name")]
    NotFoundFileName,
    #[error("This is not valid utf8")]
    InvalidUtf8,
    #[error("Incomplete conversion")]
    IncompleteConversion,
    #[error("DAR syntax error.:\n{0}")]
    InvalidDarSyntax(String),
    #[error(transparent)]
    ConditionError(#[from] crate::conditions::ConditionError),
    #[error(transparent)]
    ParseError(#[from] crate::condition_parser::ParseError),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    /// Convert json error.
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    ParseIntError(#[from] core::num::ParseIntError),
    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub type Result<T, Error = ConvertError> = core::result::Result<T, Error>;
