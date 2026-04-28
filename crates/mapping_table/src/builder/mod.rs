//! Mapping table generator for DAR-style mods.
//!
//! Provides multiple strategies to infer `(priority, name)` pairs
//! from directory structures and file names.

mod build;
mod error;
mod parser;
mod strategy;

pub use build::generate_mapping_table;
pub use error::Error;
pub use strategy::MappingStrategy;
