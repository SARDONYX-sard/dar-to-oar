use crate::{
    convert_option::{AsyncFrom, GuiConverterOptions},
    logging::change_log_level,
};
use dar2oar_core::{
    convert_dar_to_oar,
    fs::{async_closure::AsyncClosure, parallel, remove_oar, unhide_dar, ConvertOptions},
};

/// early return with Err() and write log error.
macro_rules! bail {
    ($err:expr) => {{
        tracing::error!("{}", $err);
        return Err($err.to_string());
    }};
}

#[tauri::command]
pub(crate) async fn convert_dar2oar(options: GuiConverterOptions<'_>) -> Result<String, String> {
    tracing::debug!("options: {:?}", &options);
    let run_parallel = options.run_parallel.unwrap_or_default();
    let log_level = options.log_level.as_deref().unwrap_or("error");

    let log_level = match log_level {
        "trace" | "debug" | "info" | "warn" | "error" => log_level,
        unknown_level => {
            tracing::warn!("Unknown log level {}. Fallback to error", unknown_level);
            "error"
        }
    };
    change_log_level(log_level).map_err(|err| err.to_string())?;

    let config = ConvertOptions::async_from(options).await;
    let res = match run_parallel {
        true => parallel::convert_dar_to_oar(config, AsyncClosure::default).await,
        false => convert_dar_to_oar(config, AsyncClosure::default).await,
    };
    match res {
        Ok(complete_msg) => {
            tracing::info!("{}", complete_msg);
            Ok(complete_msg.to_string())
        }
        Err(err) => bail!(err),
    }
}

#[tauri::command]
pub(crate) async fn restore_dar_dir(dar_dir: &str) -> Result<String, String> {
    match unhide_dar(dar_dir).await {
        Ok(complete_msg) => Ok(complete_msg.to_string()),
        Err(err) => bail!(err),
    }
}

#[tauri::command]
pub(crate) async fn remove_oar_dir(path: &str) -> Result<(), String> {
    remove_oar(path).await.or_else(|err| bail!(err))
}
