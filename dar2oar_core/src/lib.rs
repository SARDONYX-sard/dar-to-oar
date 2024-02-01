#![deny(clippy::all, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::derive_partial_eq_without_eq,
    clippy::future_not_send,
    clippy::multiple_crate_versions,
    clippy::pub_with_shorthand,
    clippy::redundant_pub_crate
)]
// See: https://rust-lang.github.io/rust-clippy/rust-1.75.0/index.html#/?groups=restriction
#![deny(
    clippy::allow_attributes_without_reason,
    clippy::clone_on_ref_ptr,
    clippy::disallowed_script_idents,
    clippy::error_impl_error,
    clippy::expect_used,
    clippy::filetype_is_file,
clippy::fn_to_numeric_cast_any,

    clippy::unwrap_in_result
)]
// See: https://rust-unofficial.github.io/patterns/anti_patterns/deny-warnings.html#alternatives
#![deny(
    bad_style,
    dead_code,
    future_incompatible,
    improper_ctypes,
    keyword_idents,
    missing_debug_implementations,
    missing_docs,
    no_mangle_generic_items,
    non_ascii_idents,
    non_shorthand_field_patterns,
    nonstandard_style,
    noop_method_call,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    renamed_and_removed_lints,
    trivial_casts,
    trivial_numeric_casts,
    unconditional_recursion,
    unsafe_code,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_parens,
    unused_results,
    while_true
)]
//! # `dar2oar_core`
//!
//! `dar2oar_core` is a Rust crate that provides functionality for converting Dynamic Animation Replacer (DAR) files to Overwrite Animation Replacer (OAR) files. The crate includes modules for parsing conditions, handling DAR syntax, managing values, and dealing with file systems.
//!
//! ## Examples
//!
//! ### Async with non Progress report.
//!
//! ```no_run
//! use anyhow::Result;
//! use dar2oar_core::{convert_dar_to_oar, ConvertOptions, get_mapping_table};
//! use tracing::{level_filters::LevelFilter, subscriber::DefaultGuard};
//! use tracing_appender::non_blocking::WorkerGuard;
//!
//! const DAR_DIR: &str = "../test/data/UNDERDOG Animations";
//! const TABLE_PATH: &str = "../test/settings/UnderDog Animations_v1.9.6_mapping_table.txt";
//! const LOG_PATH: &str = "../convert.log";
//!
//! /// Multithread init logger.
//! ///
//! /// File I/O is No ANSI color, output to stdout has ANSI color.
//! ///
//! /// # Returns
//! /// Guards
//! /// - If this variable is dropped, the logger stops.
//! pub(crate) fn init_tracing(
//!     test_name: &str,
//!     filter: impl Into<LevelFilter>,
//! ) -> Result<(WorkerGuard, DefaultGuard)> {
//!     use tracing_subscriber::{fmt, layer::SubscriberExt};
//!     std::fs::create_dir_all("../logs")?;
//!     let (file_writer, guard) =
//!         tracing_appender::non_blocking(std::fs::File::create(format!("../logs/{test_name}.log"))?);
//!     let thread_guard = tracing::subscriber::set_default(
//!         fmt::Subscriber::builder()
//!             .compact()
//!             .pretty()
//!             .with_file(true)
//!             .with_line_number(true)
//!             .with_max_level(filter)
//!             .with_target(false)
//!             .finish()
//!             .with(
//!                 fmt::Layer::default()
//!                     .compact()
//!                     .with_ansi(false)
//!                     .with_file(true)
//!                     .with_line_number(true)
//!                     .with_target(false)
//!                     .with_writer(file_writer),
//!             ),
//!     );
//!     Ok((guard, thread_guard))
//! }
//!
//! /// Asynchronous function to create conversion options.
//! async fn create_options() -> Result<ConvertOptions> {
//!     Ok(ConvertOptions {
//!         dar_dir: DAR_DIR.into(),
//!         section_table: get_mapping_table(Some(TABLE_PATH)).await,
//!         ..Default::default()
//!     })
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let _guard = init_tracing("unhide_dar", tracing::Level::DEBUG)?;
//!     convert_dar_to_oar(create_options().await?, |_| {}).await?;
//!     Ok(())
//! }
//! ```
//!
//! ### Parallel Async with Progress report.
//!
//! ```no_run
//! use anyhow::Result;
//! use dar2oar_core::{convert_dar_to_oar, ConvertOptions, get_mapping_table};
//!
//! const DAR_DIR: &str = "../test/data/UNDERDOG Animations";
//! const TABLE_PATH: &str = "../test/settings/UnderDog Animations_v1.9.6_mapping_table.txt";
//! const LOG_PATH: &str = "../convert.log";
//!
//! /// Initialization macro for setting up logging.
//! macro_rules! logger_init {
//!     () => {
//!         let (non_blocking, _guard) =
//!             tracing_appender::non_blocking(std::fs::File::create(LOG_PATH).unwrap());
//!         tracing_subscriber::fmt()
//!             .with_writer(non_blocking)
//!             .with_ansi(false)
//!             .with_max_level(tracing::Level::DEBUG)
//!             .init();
//!     };
//! }
//!
//! /// Asynchronous function to create conversion options.
//! async fn create_options() -> Result<ConvertOptions> {
//!     Ok(ConvertOptions {
//!         dar_dir: DAR_DIR.into(),
//!         section_table: get_mapping_table(Some(TABLE_PATH)).await,
//!         run_parallel: true,
//!         ..Default::default()
//!     })
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     logger_init!();
//!     let (tx, mut rx) = tokio::sync::mpsc::channel(500);
//!
//!     // Send function for progress reporting.
//!     let sender = move |idx: usize| {
//!         let tx = tx.clone();
//!         tokio::spawn(async move {
//!             tx.send(idx).await.unwrap_or_default();
//!         });
//!     };
//!
//!     // Spawn conversion process with progress reporting.
//!     tokio::spawn(convert_dar_to_oar(create_options().await?, sender));
//!
//!     let mut walk_len = 0usize;
//!     // Receive progress updates and print messages.
//!     while let Some(idx) = rx.recv().await {
//!          match walk_len == 0 {
//!             true => walk_len = idx, // NOTE: 1st received index is length.
//!             false => println!("[recv] Converted: {}/{}", idx + 1, walk_len),
//!          }
//!     }
//!
//!     Ok(())
//! }
//! ```
mod condition_parser;
mod conditions;
mod dar_syntax;
mod values;

pub mod error;
pub mod fs;

pub use crate::fs::converter::support_cmd::{remove_oar, unhide_dar};
pub use crate::fs::converter::{convert_dar_to_oar, Closure, ConvertOptions};
pub use crate::fs::mapping_table::{get_mapping_table, read_mapping_table};

#[cfg(test)]
mod test_helper;
#[cfg(test)]
extern crate criterion as _; // Needed for cargo bench.
