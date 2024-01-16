use std::fs::File;
use std::path::Path;
use tracing::level_filters::LevelFilter;

/// Init logger.
pub(crate) fn init_tracing(
    log_path: impl AsRef<Path>,
    filter: impl Into<LevelFilter>,
    with_stdout: bool,
) -> anyhow::Result<()> {
    use tracing_subscriber::{fmt, layer::SubscriberExt};
    let log_path = log_path.as_ref();
    if let Some(log_path) = log_path.parent() {
        std::fs::create_dir_all(log_path)?;
    };

    match with_stdout {
        true => tracing::subscriber::set_global_default(
            fmt::Subscriber::builder()
                .with_max_level(filter)
                .finish()
                .with(
                    fmt::Layer::default()
                        .with_writer(File::create(log_path)?)
                        .with_line_number(true)
                        .with_ansi(false),
                ),
        )?,
        false => tracing_subscriber::fmt()
            .with_ansi(false)
            .with_writer(File::create(log_path)?)
            .with_max_level(filter)
            .init(),
    }

    Ok(())
}
