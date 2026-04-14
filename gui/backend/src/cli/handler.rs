use dar2oar_core::{
    Closure, ConvertOptions, convert_dar_to_oar, get_mapping_table, remove_oar, unhide_dar,
};
use snafu::ResultExt as _;
use std::path::{Path, PathBuf};
use tauri::App;
use tauri_plugin_cli::CliExt as _;
use tracing::error;
use tracing::info;

use crate::cli::{
    ArgData, HashMap,
    error::{CliError, CliParseSnafu, CoreSnafu, LoggerInitSnafu, Result, RuntimeSnafu},
    get_opt_string, get_string,
};

pub fn handle_cli(app: &App) -> Result<bool, CliError> {
    let matches = app.cli().matches().context(CliParseSnafu)?;

    let Some(subcommand) = &matches.subcommand else {
        return Ok(false);
    };

    #[cfg(target_os = "windows")]
    unsafe {
        let _ = windows_sys::Win32::System::Console::AllocConsole();
    };

    init_logger(&matches)?;

    let rt = tokio::runtime::Runtime::new().context(RuntimeSnafu)?;

    let result = rt.block_on(async move {
        match subcommand.name.as_str() {
            "convert" => handle_convert(&subcommand.matches.args).await,
            "unhide-dar" => handle_unhide(&subcommand.matches.args).await,
            "remove-oar" => handle_remove(&subcommand.matches.args).await,
            _ => Ok(()),
        }
    });

    if let Err(e) = &result {
        error!(error = %e, "CLI failed");
    }

    result.map(|_| true)
}

fn init_logger(matches: &tauri_plugin_cli::Matches) -> Result<()> {
    let log_level = get_opt_string(&matches.args, "log_level");
    let log_file = get_opt_string(&matches.args, "log_file");

    let log_file = log_file.unwrap_or_else(|| "./logs/dar2oar.log".to_string());
    let log_file = Path::new(&log_file);

    let log_dir = log_file
        .parent()
        .map_or_else(|| PathBuf::from("./logs"), PathBuf::from);
    let log_name = log_file
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("dar2oar.log");

    tracing_rotation::init(&log_dir, log_name).context(LoggerInitSnafu)?;
    tracing_rotation::change_level(log_level.as_deref().unwrap_or("error"))
        .context(LoggerInitSnafu)?;

    info!("logger initialized");

    Ok(())
}

async fn handle_convert(args: &HashMap<String, ArgData>) -> Result<(), CliError> {
    let config = ConvertOptions {
        dar_dir: get_string(args, "source")?,
        oar_dir: get_opt_string(args, "destination"),
        mod_name: get_opt_string(args, "name"),
        author: get_opt_string(args, "author"),
        description: get_opt_string(args, "description"),
        section_table: get_mapping_table(get_opt_string(args, "mapping_file")).await,
        section_1person_table: get_mapping_table(get_opt_string(args, "mapping_1person_file"))
            .await,
        run_parallel: args.contains_key("run_parallel"),
        hide_dar: args.contains_key("hide_dar"),
    };

    convert_dar_to_oar(config, Closure::default)
        .await
        .context(CoreSnafu)
}

async fn handle_unhide(args: &HashMap<String, ArgData>) -> Result<(), CliError> {
    let dir = get_string(args, "dar_dir")?;

    unhide_dar(dir, Closure::default).await.context(CoreSnafu)
}

async fn handle_remove(args: &HashMap<String, ArgData>) -> Result<(), CliError> {
    let path = get_string(args, "target_path")?;

    remove_oar(path, Closure::default).await.context(CoreSnafu)
}
