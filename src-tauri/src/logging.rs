use anyhow::{bail, Context as _, Result};
use chrono::Local;
use once_cell::sync::OnceCell;
use std::fs::{self, File};
use std::path::Path;
use std::str::FromStr;
use tracing::debug;
use tracing_subscriber::{
    filter::{self, LevelFilter},
    fmt,
    prelude::*,
    reload::{self, Handle},
    Registry,
};

pub static INSTANCE: OnceCell<Handle<LevelFilter, Registry>> = OnceCell::new();

pub(crate) fn init_logger(app: &tauri::App) -> Result<Handle<LevelFilter, Registry>> {
    let resolver = app.path_resolver();
    let log_dir = &resolver.app_log_dir().context("Not found log dir")?;
    let log_name = format!("{}.log", app.package_info().name);

    let fmt_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(create_log(log_dir, &log_name, 4)?);

    let (filter, reload_handle) = reload::Layer::new(filter::LevelFilter::ERROR);
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();
    tracing::debug!("logger file path: {:?}", log_name);
    Ok(reload_handle)
}

pub(crate) fn change_log_level(log_level: &str) -> Result<()> {
    match INSTANCE.get() {
        Some(log) => log
            .modify(|filter| {
                *filter =
                    filter::LevelFilter::from_str(log_level).unwrap_or(filter::LevelFilter::ERROR)
            })
            .context("Couldn't change log level"),
        None => bail!("Uninitialized logger."),
    }
}

/// Rotation Logger File Creator.
/// - When the maximum count is reached, delete the descending ones first and create a new log file.
fn create_log(log_dir: impl AsRef<Path>, log_name: &str, max_files: usize) -> Result<File> {
    fs::create_dir_all(&log_dir)?;

    let mut log_files = fs::read_dir(&log_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_str()
                .map(|name| name.starts_with(log_name))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    debug!("existed log files: {:?}", &log_files);
    let log_file = log_dir.as_ref().join(log_name);
    if log_files.len() > max_files {
        log_files.sort_by(|a, b| {
            // NOTE: Error in OS that can't be modified, but not considered here.
            a.metadata()
                .unwrap()
                .modified()
                .unwrap()
                .cmp(&b.metadata().unwrap().modified().unwrap())
        });
        if let Some(oldest_file) = log_files.first() {
            debug!("Remove old log {:?}", &oldest_file);
            fs::remove_file(oldest_file.path())?;
        }
    } else {
        let old_file = log_dir.as_ref().join(format!(
            "{}_{}.log",
            log_name,
            Local::now().format("%F__%H_%M_%S")
        ));

        if log_file.exists() {
            debug!("From log_file: {:?}", &log_file);
            debug!("To old_file: {:?}", &old_file);
            fs::rename(&log_file, old_file)?;
        }
    };
    let f = File::create(log_file)?;
    Ok(f)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! logger_init {
        () => {
            let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
            tracing_subscriber::fmt()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_max_level(tracing::Level::DEBUG)
                .init();
        };
    }

    #[test]
    fn test() -> Result<()> {
        logger_init!();
        let log_dir = temp_dir::TempDir::new()?;
        let log_dir = log_dir.path();
        for _ in 0..5 {
            create_log(log_dir, "g_dar2oar.log", 3)?;
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        fn get_files_in_dir(dir_path: impl AsRef<Path>) -> Result<Vec<fs::DirEntry>> {
            let dir = fs::read_dir(dir_path)?;
            let files = dir
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
                .collect::<Vec<fs::DirEntry>>();
            Ok(files)
        }

        let files = get_files_in_dir(log_dir)?;
        debug!("{:?}", &files);
        assert_eq!(files.len(), 3);
        Ok(())
    }
}
