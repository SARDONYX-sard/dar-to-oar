mod common;

pub mod parallel;
pub mod sequential;
pub mod support_cmd;

use crate::error::Result;
use std::collections::HashMap;
use std::path::Path;

/// # Convert DAR to OAR
///
/// Asynchronously converts Dynamic Animation Replacer (DAR) files to Overwrite Animation Replacer (OAR) files.
///
/// ## Parameters
///
/// - `options`: A structure (`ConvertOptions`) containing various configuration options for the conversion process.
///   - `dar_dir`: Path to the DAR source directory.
///   - `oar_dir`: Optional path to the OAR destination directory. If not provided, it is inferred from the source directory.
///   - `mod_name`: Optional module name in `config.json` and directory name. If not provided, it is inferred from the source directory.
///   - `author`: Optional mod author in `config.json`.
///   - `section_table`: Optional path to the section name table.
///   - `section_1person_table`: Optional path to the section name table for the first person.
///   - `run_parallel`: A boolean flag indicating whether to use multi-threading for the conversion process.
///   - `hide_dar`: A boolean flag indicating whether to add `mohidden` to the DAR directory before conversion, treating it as a hidden directory (for MO2 users).
///
/// - `progress_fn`: A closure that takes a `usize` parameter, representing the progress of the conversion.
///
/// ## Returns
///
/// - `Result<ConvertedReport>`: A result indicating the success or failure of the conversion process, along with a `ConvertedReport` enum providing details on the completed actions.
///
/// ## Examples
///
/// ### Sequential Conversion
///
/// ```no_run
/// use dar2oar_core::{convert_dar_to_oar, ConvertOptions};
///
/// #[tokio::main]
/// async fn main() {
///     let options = ConvertOptions {
///         dar_dir: "path/to/dar_directory",
///         // Specify other options as needed
///         ..Default::default()
///     };
///
///     let result = convert_dar_to_oar(options, |_| {}).await;
///     match result {
///         Ok(report) => println!("Conversion Report: {:?}", report),
///         Err(err) => eprintln!("Conversion Error: {}", err),
///     }
/// }
/// ```
///
/// ### Parallel Conversion
///
/// ```no_run
/// use dar2oar_core::{convert_dar_to_oar, ConvertOptions};
///
/// #[tokio::main]
/// async fn main() {
///     let options = ConvertOptions {
///         dar_dir: "path/to/dar_directory",
///         run_parallel: true,
///         // Specify other options as needed
///         ..Default::default()
///     };
///
///     let result = convert_dar_to_oar(options, |_| {}).await;
///     match result {
///         Ok(report) => println!("Conversion Report: {:?}", report),
///         Err(err) => eprintln!("Conversion Error: {}", err),
///     }
/// }
/// ```
pub async fn convert_dar_to_oar(
    options: ConvertOptions<'_, impl AsRef<Path>>,
    progress_fn: impl FnMut(usize),
) -> Result<ConvertedReport> {
    match options.run_parallel {
        true => crate::fs::converter::parallel::convert_dar_to_oar(options, progress_fn).await,
        false => crate::fs::converter::sequential::convert_dar_to_oar(options, progress_fn).await,
    }
}

pub struct Closure;
impl Closure {
    pub fn default(_: usize) {}
}

#[derive(Debug, Clone, Default)]
pub struct ConvertOptions<'a, P: AsRef<Path>> {
    /// DAR source dir path
    pub dar_dir: P,
    /// OAR destination dir path(If not, it is inferred from src)
    pub oar_dir: Option<P>,
    /// mod name in config.json & directory name(If not, it is inferred from src)
    pub mod_name: Option<&'a str>,
    /// mod author in config.json
    pub author: Option<&'a str>,
    /// path to section name table
    pub section_table: Option<HashMap<String, String>>,
    /// path to section name table(For _1st_person)
    pub section_1person_table: Option<HashMap<String, String>>,
    /// use multi thread(Probably effective for those with long DAR syntax. Basically single-threaded is faster.)
    pub run_parallel: bool,
    /// After converting to OAR, add mohidden to the DAR directory before conversion to treat it as a hidden directory. (for MO2 users)
    pub hide_dar: bool,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ConvertedReport {
    #[error("Conversion Completed.")]
    Complete,

    #[error("Converted & Renamed 1st, 3rd person DAR")]
    Renamed1rdAnd3rdPersonDar,
    #[error("Converted & Renamed 1rd person DAR")]
    Renamed1rdPersonDar,
    #[error("Converted & Renamed 3rd person DAR")]
    Renamed3rdPersonDar,

    #[error("Unhide 1st & 3rd person")]
    Unhide1rdAnd3rdPerson,
    #[error("Unhide 1rd person")]
    Unhide1rdPerson,
    #[error("Unhide 3rd person")]
    Unhide3rdPerson,
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    const DAR_DIR: &str = "../test/data/UNDERDOG Animations";
    const TABLE_PATH: &str = "../test/settings/UnderDog Animations_v1.9.6_mapping_table.txt";
    const LOG_PATH: &str = "../convert.log";

    /// NOTE: It is a macro because it must be called at the root of a function to function.
    macro_rules! logger_init {
        () => {
            let (non_blocking, _guard) =
                tracing_appender::non_blocking(std::fs::File::create(LOG_PATH)?);
            tracing_subscriber::fmt()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_max_level(tracing::Level::DEBUG)
                .init();
        };
    }

    async fn create_options<'a>() -> Result<ConvertOptions<'a, &'a str>> {
        Ok(ConvertOptions {
            dar_dir: DAR_DIR,
            // cannot use include_str!
            section_table: Some(crate::read_mapping_table(TABLE_PATH).await?),
            // run_parallel: true,
            ..Default::default()
        })
    }

    #[ignore]
    #[tokio::test]
    async fn convert_non_mpsc() -> Result<()> {
        logger_init!();
        convert_dar_to_oar(create_options().await?, |_| {}).await?;
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn convert_with_mpsc() -> Result<()> {
        use once_cell::sync::Lazy;
        use std::sync::atomic::AtomicUsize;
        use std::sync::atomic::Ordering;

        logger_init!();
        let (tx, mut rx) = tokio::sync::mpsc::channel(500);

        //? NOTE: Since recv does not seem to be possible until io is finished, send is used to see the output.
        let sender = move |idx: usize| {
            let tx = tx.clone();
            tokio::spawn(async move {
                tx.send(idx).await.unwrap_or_default();
            });
        };

        let handle = tokio::spawn(convert_dar_to_oar(create_options().await?, sender));
        while let Some(idx) = rx.recv().await {
            static NUM: Lazy<AtomicUsize> = Lazy::new(AtomicUsize::default);
            let num = NUM.load(Ordering::Acquire);
            if num != 0 {
                println!("[recv] Converted: {}/{}", idx, num);
            } else {
                NUM.store(idx, Ordering::Release);
                println!("[recv] Converted: {}", idx);
            }
        }

        let _ = handle.await.unwrap();
        Ok(())
    }
}
