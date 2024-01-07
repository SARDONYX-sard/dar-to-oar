//! # dar2oar_core
//!
//! `dar2oar_core` is a Rust crate that provides functionality for converting Dynamic Animation Replacer (DAR) files to Overwrite Animation Replacer (OAR) files. The crate includes modules for parsing conditions, handling DAR syntax, managing values, and dealing with file systems.
//!
//! ## Modules
//!
//! - [condition_parser](condition_parser): Module to convert a parsed DAR into a serializable OAR structure.
//! - [conditions](conditions): Module for representing conditions used in DAR files.
//! - [dar_syntax](dar_syntax): Module for handling DAR syntax and conversions.
//! - [values](values): Module for managing values used in OAR files.
//!
//! ## Submodules
//!
//! - [error](error): Submodule for defining error types related to DAR to OAR conversion.
//! - [fs](fs): Submodule containing file system-related functionalities, including the main conversion function.
//!
//! ## Public Functions and Types
//!
//! - `convert_dar_to_oar`: The main function for converting DAR files to OAR files. It accepts configuration options and a progress callback.
//! - `Closure`: A struct that provides a default closure for progress reporting.
//! - `ConvertOptions`: A struct containing various configuration options for the conversion process.
//! - `ConvertedReport`: An enum representing different outcomes of the conversion process.
//! - `remove_oar`: Function for removing OAR files from a directory.
//! - `unhide_dar`: Function to unhide DAR files after conversion.
//! - `get_mapping_table`: Function for obtaining a mapping table.
//! - `read_mapping_table`: Function for reading a mapping table from a specified path.
//!
//! ## Examples
//!
//! ### Async with non Progress report.
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
//!         ..Default::default()
//!     })
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     logger_init!();
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
//!
mod condition_parser;
mod conditions;
mod dar_syntax;
mod values;

pub mod error;
pub mod fs;

pub use crate::fs::converter::support_cmd::{remove_oar, unhide_dar};
pub use crate::fs::converter::{convert_dar_to_oar, Closure, ConvertOptions, ConvertedReport};
pub use crate::fs::mapping_table::{get_mapping_table, read_mapping_table};
