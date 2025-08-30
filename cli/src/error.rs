//! errors of `This crate`
use std::{io, path::PathBuf};

/// GUI Error
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    /// Standard io error
    #[snafu(transparent)]
    FailedIo { source: io::Error },

    #[snafu(transparent)]
    FailedConvert {
        source: dar2oar_core::error::ConvertError,
    },

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Tracing log error
    #[snafu(transparent)]
    UnableSetTracing {
        source: tracing::subscriber::SetGlobalDefaultError,
    },

    /// Tracing subscriber reload error
    #[snafu(transparent)]
    FailedReloadTracingSub {
        source: tracing_subscriber::reload::Error,
    },
}

/// `Result` for this crate.
pub type Result<T, E = Error> = core::result::Result<T, E>;
