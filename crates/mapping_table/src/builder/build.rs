use crate::MappingTable;

use super::error::Error;
use super::parser::{parse_dir_pattern, strip_numbers};
use super::strategy::MappingStrategy;

use std::path::Path;

/// Generate mapping table from a directory.
///
/// # Errors
///
/// Returns an error if the path does not exist or if priority extraction fails.
///
/// Root example:
///   .../meshes/actors/character/animations/DynamicAnimationReplacer
pub fn generate_mapping_table(
    path: &Path,
    strategy: MappingStrategy,
) -> Result<MappingTable, Error> {
    let root = path;

    if !root.exists() {
        return Err(Error::PathNotFound {
            path: path.to_path_buf(),
        });
    }

    let mut map = MappingTable::new();

    for entry in jwalk::WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let path = path.as_path();

        // ----------------------
        // Skip everything until we are under DynamicAnimationReplacer
        //
        // Expected:
        //   .../DynamicAnimationReplacer/...
        //
        if !is_under_dar(path) {
            continue;
        }

        // ----------------------
        // Detect priority directory (DAR)
        //
        // Expected:
        //   .../DynamicAnimationReplacer/<container>/<priority>/
        //                                                ^ this directory
        //
        // <container>:
        //   - plugin name (e.g. "Skyrim.esm")
        //   - or "_CustomConditions"
        //
        if let Some(priority) = get_priority(path) {
            map.entry(priority.to_string()).or_default();
        }

        match strategy {
            MappingStrategy::TxtStem => {
                if !is_rename_source_txt(path) {
                    continue;
                }

                let priority = get_priority(path).ok_or_else(|| Error::InvalidPriority {
                    path: path.to_path_buf(),
                })?;

                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();

                map.insert(priority.to_string(), name);
            }

            MappingStrategy::TxtStemStripped => {
                if !is_rename_source_txt(path) {
                    continue;
                }

                let priority = get_priority(path).ok_or_else(|| Error::InvalidPriority {
                    path: path.to_path_buf(),
                })?;

                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(strip_numbers)
                    .unwrap_or_default();

                map.insert(priority.to_string(), name);
            }

            MappingStrategy::DirPattern => {
                if !path.is_dir() {
                    continue;
                }

                let Some(dir_name) = path.file_name().and_then(|s| s.to_str()) else {
                    continue;
                };
                if dir_name.eq_ignore_ascii_case("DynamicAnimationReplacer")
                    || dir_name.eq_ignore_ascii_case("_CustomConditions")
                {
                    continue;
                }

                if let Some((priority, name)) = parse_dir_pattern(dir_name) {
                    if !is_valid_priority(priority) {
                        continue;
                    }

                    #[cfg(feature = "tracing")]
                    tracing::debug!(priority, name, "parsed dir pattern");

                    map.insert(priority.to_string(), name.to_string());
                }
            }
        }
    }

    Ok(map)
}

// ----------------------
// Helper functions
// ----------------------
/// Check if path is under DynamicAnimationReplacer.
///
/// Expected:
///   .../DynamicAnimationReplacer/...
fn is_under_dar(path: &Path) -> bool {
    path.components().any(|c| {
        c.as_os_str()
            .to_str()
            .is_some_and(|s| s.eq_ignore_ascii_case("DynamicAnimationReplacer"))
    })
}

/// Extract priority from DAR-like path.
///
/// Expected structure:
///   .../DynamicAnimationReplacer/<container>/<priority>/<file_or_dir>
///
/// <container>:
///   - plugin (e.g. "Skyrim.esm")
///   - or "_CustomConditions"
///
/// Examples:
///   .../DynamicAnimationReplacer/Skyrim.esm/666003/foo.txt
///   .../DynamicAnimationReplacer/_CustomConditions/666003/_conditions.txt
///
/// returns "666003"
fn get_priority(path: &Path) -> Option<&str> {
    let mut components = path.components().rev();

    // file or dir (skip)
    components.next()?;

    // priority dir
    let priority = components.next()?.as_os_str().to_str()?;

    // skip container (plugin or special dir)
    let _ = components.next()?.as_os_str().to_str()?;

    // must be under DynamicAnimationReplacer
    let dar = components.next()?.as_os_str().to_str()?;
    if !dar.eq_ignore_ascii_case("DynamicAnimationReplacer") {
        return None;
    }

    if is_valid_priority(priority) {
        Some(priority)
    } else {
        #[cfg(feature = "tracing")]
        tracing::debug!(priority, container, "invalid priority detected");
        None
    }
}

/// Valid priority:
///   - decimal only (666003)
///   - hex only (1A2B)
fn is_valid_priority(s: &str) -> bool {
    let is_decimal = s.chars().all(|c| c.is_ascii_digit());
    let is_hex = s.chars().all(|c| c.is_ascii_hexdigit());

    is_decimal || is_hex
}

/// Check if this txt should be used as rename source.
///
/// Expected:
///   .../DynamicAnimationReplacer/<container>/<priority>/foo.txt
///
/// Excludes:
///   .../_conditions.txt
fn is_rename_source_txt(path: &Path) -> bool {
    path.extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("txt"))
        && !path
            .file_name()
            .and_then(|s| s.to_str())
            .is_some_and(|s| s.eq_ignore_ascii_case("_conditions.txt"))
        && get_priority(path).is_some()
}
