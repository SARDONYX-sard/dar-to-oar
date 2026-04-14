//! errors of `This crate`

/// GUI Error
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    /// Standard io error
    #[snafu(transparent)]
    FailedIo { source: std::io::Error },

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Logger
    /// Not found log dir. {source}
    NotFoundLogDir { source: tauri::Error },

    /// Tracing log error
    #[snafu(transparent)]
    FailedReloadTracingSub {
        source: tracing_rotation::error::Error,
    },
}

/// `Result` for this crate.
pub type Result<T, E = Error> = core::result::Result<T, E>;
