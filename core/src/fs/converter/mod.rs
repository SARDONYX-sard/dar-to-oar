//! Converter system
mod common;

pub mod parallel;
pub mod sequential;
pub mod support_cmd;

use crate::error::Result;
use compact_str::CompactString;
use std::collections::HashMap;

/// Converts Dynamic Animation Replacer (DAR) files to Overwrite Animation Replacer (OAR) files.
///
/// # Errors
/// Failed conversion
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
    let dar_dir = std::path::Path::new(&options.dar_dir);
    if !dar_dir.exists() {
        return Err(crate::error::ConvertError::NonExistPath(format!(
            "{dar_dir:?}"
        )));
    };

    match options.run_parallel {
        true => crate::fs::converter::parallel::convert_dar_to_oar(options, progress_fn).await,
        false => crate::fs::converter::sequential::convert_dar_to_oar(options, progress_fn).await,
    }
}

/// A structure for creating dummy functions to facilitate refactoring.
#[derive(Debug)]
pub struct Closure;
impl Closure {
    /// No operation function pointer
    #[inline]
    pub const fn default(_: usize) {}
}

/// The options for converting a DAR directory to an OAR directory.
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
    pub section_table: Option<HashMap<CompactString, String>>,
    /// path to section name table(For `_1st_person`)
    pub section_1person_table: Option<HashMap<CompactString, String>>,
    /// use multi thread(Probably effective for those with long DAR syntax. Basically single-threaded is faster.)
    pub run_parallel: bool,
    /// After converting to OAR, add mohidden to the DAR directory before conversion to treat it as a hidden directory. (for MO2 users)
    pub hide_dar: bool,
}

#[cfg(feature = "tracing")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::error::Result;

    // const DAR_DIR: &str = "../test/data/UNDERDOG - Animations";
    // const DAR_DIR: &str = "../test/data/Delia";
    const DAR_DIR: &str = "../test/data/Axarien's Animations - The Companions (DAR)";
    // const OAR_DIR: &str =
    //     "../test/data/Delia/meshes/actors/character/animations\\OpenAnimationReplacer";
    // const TABLE_PATH: &str = "../test/mapping_tables/UnderDog Animations_v1.9.6_mapping_table.txt";

    async fn create_options() -> Result<ConvertOptions> {
        Ok(ConvertOptions {
            dar_dir: DAR_DIR.into(),
            // oar_dir: Some(OAR_DIR.into()),
            // cannot use include_str!
            // section_table: Some(crate::read_mapping_table(TABLE_PATH).await?),
            // run_parallel: true,
            // hide_dar: true,
            ..Default::default()
        })
    }

    #[ignore = "need MOD data"]
    #[tokio::test]
    #[cfg_attr(
        feature = "tracing",
        quick_tracing::try_init(test = "convert_non_mpsc", level = "DEBUG")
    )]
    async fn convert_non_mpsc() -> Result<()> {
        convert_dar_to_oar(create_options().await?, |_| {}).await?;
        Ok(())
    }

    #[ignore = "need MOD data"]
    #[tokio::test]
    #[cfg_attr(
        feature = "tracing",
        quick_tracing::try_init(test = "convert_mpsc", level = "DEBUG")
    )]
    async fn convert_with_mpsc() -> Result<()> {
        let (tx, mut rx) = tokio::sync::mpsc::channel(500);

        let sender = move |idx: usize| {
            let tx = tx.clone();
            let handle = tokio::spawn(async move {
                match tx.send(idx).await {
                    Ok(ok) => ok,
                    Err(err) => tracing::error!("{}", err),
                };
            });

            drop(handle);
        };

        let handle = tokio::spawn(convert_dar_to_oar(create_options().await?, sender));

        let mut walk_len = 0;
        while let Some(idx) = rx.recv().await {
            if walk_len == 0 {
                walk_len = idx;
            } else {
                tracing::info!("[recv] Converted: {}/{walk_len}", idx + 1);
            }
        }

        handle.await??;
        Ok(())
    }
}
