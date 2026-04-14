#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum CliError {
    #[snafu(display("CLI parse failed: {source}"))]
    CliParse { source: tauri_plugin_cli::Error },

    #[snafu(display("Missing argument: {key}"))]
    MissingArg { key: String },

    #[snafu(display("Core error: {source}"))]
    Core {
        source: dar2oar_core::error::ConvertError,
    },

    #[snafu(display("Tokio runtime execute failed: {source}"))]
    Runtime { source: std::io::Error },

    /// Tracing log error
    #[snafu(display("Logger init failed: {source}"))]
    LoggerInit {
        source: tracing_rotation::error::Error,
    },
}

pub type Result<T, E = CliError> = std::result::Result<T, E>;
