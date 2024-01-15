mod common;

pub mod parallel;
pub mod sequential;
pub mod support_cmd;

use crate::error::Result;
use std::collections::HashMap;

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
///         dar_dir: "path/to/dar_directory".into(),
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
///         dar_dir: "path/to/dar_directory".into(),
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
    options: ConvertOptions,
    progress_fn: impl FnMut(usize),
) -> Result<()> {
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
pub struct ConvertOptions {
    /// DAR source dir path
    pub dar_dir: String,
    /// OAR destination dir path(If not, it is inferred from src)
    pub oar_dir: Option<String>,
    /// mod name in config.json & directory name(If not, it is inferred from src)
    pub mod_name: Option<String>,
    /// mod author in config.json
    pub author: Option<String>,
    /// path to section name table
    pub section_table: Option<HashMap<String, String>>,
    /// path to section name table(For _1st_person)
    pub section_1person_table: Option<HashMap<String, String>>,
    /// use multi thread(Probably effective for those with long DAR syntax. Basically single-threaded is faster.)
    pub run_parallel: bool,
    /// After converting to OAR, add mohidden to the DAR directory before conversion to treat it as a hidden directory. (for MO2 users)
    pub hide_dar: bool,
}

#[cfg(test)]
mod test {
    use crate::test_helper::init_tracing;

    use super::*;
    use anyhow::Result;

    const DAR_DIR: &str = "../test/data/UNDERDOG Animations";
    const TABLE_PATH: &str = "../test/settings/UnderDog Animations_v1.9.6_mapping_table.txt";

    async fn create_options() -> Result<ConvertOptions> {
        Ok(ConvertOptions {
            dar_dir: DAR_DIR.into(),
            // cannot use include_str!
            section_table: Some(crate::read_mapping_table(TABLE_PATH).await?),
            run_parallel: true,
            ..Default::default()
        })
    }

    #[ignore]
    #[tokio::test]
    async fn convert_non_mpsc() -> Result<()> {
        let _guard = init_tracing("convert_non_mpsc", tracing::Level::DEBUG)?;
        convert_dar_to_oar(create_options().await?, |_| {}).await?;
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn convert_with_mpsc() -> Result<()> {
        let _guard = init_tracing("convert_non_mpsc", tracing::Level::DEBUG)?;
        let (tx, mut rx) = tokio::sync::mpsc::channel(500);

        let sender = move |idx: usize| {
            let tx = tx.clone();
            tokio::spawn(async move {
                tx.send(idx).await.unwrap();
            });
        };

        let handle = tokio::spawn(convert_dar_to_oar(create_options().await?, sender));

        let mut walk_len = 0usize;
        while let Some(idx) = rx.recv().await {
            match walk_len == 0 {
                true => walk_len = idx, // NOTE: 1st received index is length.
                false => println!("[recv] Converted: {}/{}", idx + 1, walk_len),
            }
        }

        let _guard = handle.await.unwrap();
        Ok(())
    }
}
