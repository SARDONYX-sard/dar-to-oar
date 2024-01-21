use anyhow::Result;
use tracing::{level_filters::LevelFilter, subscriber::DefaultGuard};
use tracing_appender::non_blocking::WorkerGuard;

/// multithread init logger.
///
/// # Returns
/// Guards
/// - If this variable is dropped, the logger stops.
pub(crate) fn init_tracing(
    test_name: &str,
    filter: impl Into<LevelFilter>,
) -> Result<(WorkerGuard, DefaultGuard)> {
    use tracing_subscriber::{fmt, layer::SubscriberExt};
    std::fs::create_dir_all("../logs")?;
    let (file_writer, guard) =
        tracing_appender::non_blocking(std::fs::File::create(format!("../logs/{test_name}.log"))?);
    let thread_guard = tracing::subscriber::set_default(
        fmt::Subscriber::builder()
            .compact()
            .pretty()
            .with_file(true)
            .with_line_number(true)
            .with_max_level(filter)
            .with_target(false)
            .with_thread_ids(true)
            .finish()
            .with(
                fmt::Layer::default()
                    .compact()
                    .with_ansi(false)
                    .with_file(true)
                    .with_line_number(true)
                    .with_target(false)
                    .with_thread_ids(true)
                    .with_writer(file_writer),
            ),
    );
    Ok((guard, thread_guard))
}
