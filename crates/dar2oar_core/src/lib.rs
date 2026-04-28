// See: https://rust-unofficial.github.io/patterns/anti_patterns/deny-warnings.html#alternatives
//! # `dar2oar_core`
//!
//! `dar2oar_core` is a Rust crate that provides functionality for converting Dynamic Animation Replacer (DAR) files to Overwrite Animation Replacer (OAR) files. The crate includes modules for parsing conditions, handling DAR syntax, managing values, and dealing with file systems.
//!
//! ## Examples
//!
//! ### Async with non Progress report.
//!
//! ```no_run
//! use dar2oar_core::{convert_dar_to_oar, ConvertOptions, error::Result, get_mapping_table};
//! use tracing::{level_filters::LevelFilter, subscriber::DefaultGuard};
//!
//! const DAR_DIR: &str = "../test/data/UNDERDOG Animations";
//! const TABLE_PATH: &str = "../test/settings/UnderDog Animations_v1.9.6_mapping_table.txt";
//!
//! /// Asynchronous function to create conversion options.
//! async fn create_options() -> Result<ConvertOptions> {
//!     Ok(ConvertOptions {
//!         dar_dir: DAR_DIR.into(),
//!         section_table: read_mapping_table(TABLE_PATH).await,
//!         ..Default::default()
//!     })
//! }
//!
//! #[tokio::main]
//! #[quick_tracing::try_init(file = "../convert.log", level = "DEBUG")]
//! async fn main() -> Result<()> {
//!     convert_dar_to_oar(create_options().await?, |_| {}).await?;
//!     Ok(())
//! }
//! ```
//!
//! ### Parallel Async with Progress report.
//!
//! ```no_run
//! use dar2oar_core::{convert_dar_to_oar, ConvertOptions, error::Result, read_mapping_table};
//!
//! const DAR_DIR: &str = "../test/data/UNDERDOG Animations";
//! const TABLE_PATH: &str = "../test/settings/UnderDog Animations_v1.9.6_mapping_table.txt";
//!
//! /// Asynchronous function to create conversion options.
//! async fn create_options() -> Result<ConvertOptions> {
//!     Ok(ConvertOptions {
//!         dar_dir: DAR_DIR.into(),
//!         section_table: read_mapping_table(TABLE_PATH).await,
//!         run_parallel: true,
//!         ..Default::default()
//!     })
//! }
//!
//! #[tokio::main]
//! #[quick_tracing::try_init(file = "../convert.log", level = "DEBUG")]
//! async fn main() -> Result<()> {
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
//!
//! ```
mod parser;

pub mod error;
pub mod fs;

#[doc = include_str!("../readme.md")]
pub use crate::fs::converter::support_cmd::{remove_oar, unhide_dar};
pub use crate::fs::converter::{Closure, ConvertOptions, convert_dar_to_oar};
pub use crate::fs::mapping_table::read_mapping_table;

#[cfg(test)]
extern crate criterion as _; // Needed for cargo bench.

#[cfg(test)]
extern crate quick_tracing as _; // To avoid lint error.
#[cfg(test)]
extern crate temp_dir as _; // To avoid lint error.
