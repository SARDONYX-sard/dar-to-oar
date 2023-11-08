use crate::convert_option::{AsyncFrom, GuiConverterOptions};
use dar2oar_core::{
    convert_dar_to_oar,
    fs::{async_closure::AsyncClosure, parallel, remove_oar, unhide_dar, ConvertOptions},
};
use tauri::Window;

/// logger hook or bail!
macro_rules! response {
    ($res:expr) => {
        match $res {
            Ok(msg) => {
                tracing::info!("{}", msg);
                Ok(msg.to_string())
            }
            Err(err) => bail!(err),
        }
    };
}

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
    let config = ConvertOptions::async_from(options).await;

    let res = match run_parallel {
        true => parallel::convert_dar_to_oar(config, AsyncClosure::default).await,
        false => convert_dar_to_oar(config, AsyncClosure::default).await,
    };
    response!(res)
}

#[tauri::command]
pub(crate) async fn convert_dar2oar_with_progress(
    window: Window,
    options: GuiConverterOptions<'_>,
) -> Result<String, String> {
    tracing::debug!("options: {:?}", &options);
    let run_parallel = options.run_parallel.unwrap_or_default();

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct Payload {
        index: usize,
    }
    let sender = move |index: usize| {
        if let Err(err) = window.emit("show-progress", Payload { index }) {
            tracing::error!("{}", err);
        };
        async move {}
    };
    let config = ConvertOptions::async_from(options).await;

    let res = match run_parallel {
        true => parallel::convert_dar_to_oar(config, sender).await,
        false => convert_dar_to_oar(config, sender).await,
    };
    response!(res)
}

#[tauri::command]
pub(crate) async fn change_log_level(log_level: Option<&str>) -> Result<(), String> {
    tracing::debug!("Selected log level: {:?}", log_level);
    let log_level = log_level.unwrap_or("error");
    let log_level = match log_level {
        "trace" | "debug" | "info" | "warn" | "error" => log_level,
        unknown_level => {
            tracing::warn!("Unknown log level {}. Fallback to error", unknown_level);
            "error"
        }
    };
    crate::logging::change_log_level(log_level).or_else(|err| bail!(err))
}

#[tauri::command]
pub(crate) async fn restore_dar_dir(dar_dir: &str) -> Result<String, String> {
    response!(unhide_dar(dar_dir).await)
}

#[tauri::command]
pub(crate) async fn remove_oar_dir(path: &str) -> Result<(), String> {
    remove_oar(path).await.or_else(|err| bail!(err))
}
