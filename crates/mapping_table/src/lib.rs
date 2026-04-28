//! Mapping table generator for DAR-style mods.
//!
//! Provides multiple strategies to infer `(priority, name)` pairs
//! from directory structures and file names.

pub mod builder;
pub mod reader;

pub type MappingTable = std::collections::HashMap<String, String>;
