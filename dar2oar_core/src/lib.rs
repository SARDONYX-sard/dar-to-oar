#![deny(clippy::all, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::derive_partial_eq_without_eq,
    clippy::future_not_send,
    clippy::multiple_crate_versions,
    clippy::pub_with_shorthand,
    clippy::redundant_pub_crate
)]
#![warn(
    clippy::dbg_macro,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::todo,
    clippy::unimplemented
)]
// See: https://rust-lang.github.io/rust-clippy/rust-1.75.0/index.html#/?groups=restriction
#![deny(
    clippy::allow_attributes_without_reason,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::clone_on_ref_ptr,
    clippy::cognitive_complexity,
    clippy::debug_assert_with_mut_call,
    clippy::disallowed_script_idents,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::enum_glob_use,
    clippy::error_impl_error,
    clippy::exit,
    clippy::expect_used,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::filter_map_next,
    clippy::flat_map_option,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::float_equality_without_abs,
    clippy::fn_params_excessive_bools,
    clippy::fn_to_numeric_cast_any,
    clippy::from_iter_instead_of_collect,
    clippy::if_let_mutex,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::items_after_test_module,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wild_err_arm,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::missing_enforced_import_renames,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::option_if_let_else,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::rc_mutex,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::semicolon_if_nothing_returned,
    clippy::single_match_else,
    clippy::string_add,
    clippy::string_add_assign,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::suspicious_operation_groupings,
    clippy::trait_duplication_in_bounds,
    clippy::unnested_or_patterns,
    clippy::unseparated_literal_suffix,
    clippy::unused_self,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::used_underscore_binding,
    clippy::useless_let_if_seq,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::wildcard_dependencies,
    clippy::wildcard_imports,
    clippy::zero_sized_map_values,
    clippy::await_holding_lock
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
//!
//! const DAR_DIR: &str = "../test/data/UNDERDOG Animations";
//! const TABLE_PATH: &str = "../test/settings/UnderDog Animations_v1.9.6_mapping_table.txt";
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
//! use anyhow::Result;
//! use dar2oar_core::{convert_dar_to_oar, ConvertOptions, get_mapping_table};
//!
//! const DAR_DIR: &str = "../test/data/UNDERDOG Animations";
//! const TABLE_PATH: &str = "../test/settings/UnderDog Animations_v1.9.6_mapping_table.txt";
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
mod condition_parser;
mod conditions;
mod dar_syntax;
mod values;

pub mod error;
pub mod fs;

#[doc = include_str!("../readme.md")]
pub use crate::fs::converter::support_cmd::{remove_oar, unhide_dar};
pub use crate::fs::converter::{convert_dar_to_oar, Closure, ConvertOptions};
pub use crate::fs::mapping_table::{get_mapping_table, read_mapping_table};

#[cfg(test)]
extern crate criterion as _; // Needed for cargo bench.

#[cfg(test)]
extern crate quick_tracing as _; // To avoid lint error.
#[cfg(test)]
extern crate temp_dir as _; // To avoid lint error.
