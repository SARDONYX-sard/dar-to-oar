#[derive(Debug, thiserror::Error)]
pub enum ConvertError {
    #[error("Nothing in the specified path")]
    NotFoundEntry,
    #[error("Failed to write section config target: {0}")]
    FailedWriteSectionConfig(String),
    #[error("Neither 1st or 3rd person \"DynamicAnimationReplacer.mohidden\" found")]
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
    #[error("Incomplete parse DAR. Remain:\n{0}")]
    IncompleteParseDar(String),
    #[error("DAR syntax error.:\n{0}")]
    InvalidDarSyntax(String),
    #[error(transparent)]
    ConditionError(#[from] crate::conditions::ConditionError),
    #[error(transparent)]
    ParseError(#[from] crate::condition_parser::ParseError),
    /// Convert json error.
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    ParseIntError(#[from] core::num::ParseIntError),
    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

//? Implemented to facilitate testing with the `assert_eq!` macro.
impl PartialEq for ConvertError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::FailedWriteSectionConfig(l0), Self::FailedWriteSectionConfig(r0)) => l0 == r0,
            (Self::InvalidDarSyntax(l0), Self::InvalidDarSyntax(r0)) => l0 == r0,
            (Self::ConditionError(l0), Self::ConditionError(r0)) => l0 == r0,
            (Self::ParseError(l0), Self::ParseError(r0)) => l0 == r0,
            (Self::JsonError(l0), Self::JsonError(r0)) => l0.to_string() == r0.to_string(),
            (Self::ParseIntError(l0), Self::ParseIntError(r0)) => l0 == r0,
            (Self::IOError(l0), Self::IOError(r0)) => l0.kind() == r0.kind(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

pub type Result<T, Error = ConvertError> = core::result::Result<T, Error>;
