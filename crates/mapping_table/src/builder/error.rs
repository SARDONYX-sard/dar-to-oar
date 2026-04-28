/// Errors that can occur during mapping table generation.
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    #[snafu(display("Path does not exist: {}", path.display()))]
    PathNotFound { path: std::path::PathBuf },

    #[snafu(display("Failed to parse priority from path: {}", path.display()))]
    InvalidPriority { path: std::path::PathBuf },
}
