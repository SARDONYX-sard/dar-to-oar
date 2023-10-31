use anyhow::{bail, Context as _, Result};
use once_cell::sync::OnceCell;
use std::{fs::File, str::FromStr};
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
    let log_path = &resolver.app_log_dir().context("Not found log dir")?;
    let log_path = log_path.join(format!("{}.log", app.package_info().name));
    let file = File::create(&log_path)?;
    let fmt_layer = fmt::layer().with_ansi(false).with_writer(file);

    let (filter, reload_handle) = reload::Layer::new(filter::LevelFilter::ERROR);
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();
    tracing::debug!("logger file path: {:?}", log_path);
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
