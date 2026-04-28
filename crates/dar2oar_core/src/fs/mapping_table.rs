//! Module for handling mapping tables in the conversion process.
//!
//! This module provides functions to read and parse mapping tables from files asynchronously.
use crate::error::{ConvertError, Result};
use mapping_table::{MappingTable, reader::parse_mapping_table};
use std::path::Path;
use tokio::fs::read_to_string;

/// Try to read mapping table from path
///
/// # Errors
/// Path is not exist.
pub async fn read_mapping_table<P>(table_path: P) -> Result<MappingTable>
where
    P: AsRef<Path>,
{
    let table_path = table_path.as_ref();
    if !table_path.exists() {
        return Err(ConvertError::NonExistPath {
            path: table_path.to_path_buf(),
        });
    };

    let contents = read_to_string(table_path).await?;
    let table =
        parse_mapping_table(&contents).map_err(|source| ConvertError::MappingTableError {
            path: table_path.to_path_buf(),
            source,
        })?;

    Ok(table)
}
