//! The path of the DAR is analyzed to obtain the information necessary for the conversion.
//!
//! # Format
//! - Common: "\<ABS or related `ParentDir`\>/\<`ModName`\>/meshes/actors/character/\[_1stperson\]/animations/"
//!
//! ### OAR:([]: optional, <>: variable)
//! - Common + "`OptionAnimationReplacer`/\<`NameSpace`\>/\<`EachSectionName`\>"
//!
//! ### DAR: (Only priority order assignment is taken into account. In other words, actor-based allocation is not considered.)
//! - Common + "`DynamicAnimationReplacer`/`_CustomConditions`/\<priority\>/_conditions.txt"

use crate::error::{ConvertError, Result};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// The information necessary for the conversion
#[derive(Debug, Clone)]
pub struct ParsedPath {
    /// ModName/meshes/actors/character/animations/DynamicAnimationReplacer
    pub dar_root: PathBuf,
    /// ModName/meshes/actors/character/animations/OpenAnimationReplacer
    pub oar_root: PathBuf,
    /// A path that contains a directory named `_1stperson`?
    pub is_1st_person: bool,
    /// ModName/meshes/actors/character
    pub mod_name: Option<String>,
    /// Number is the expected priority dir name of the formal DAR,
    /// but returns Err for the Mod creator who leaves a note.
    pub priority: Result<i32, String>,
    /// male, female dir
    pub remain_dir: Option<PathBuf>,
}

/// Parses the DAR path and returns the information necessary for conversion.
///
/// # Errors
/// - If `DynamicAnimationReplacer` is not found in path
/// - `priority`: if `_CustomConditions` is not found
///
/// # When does the return value return None?
/// Each of the following will be None if not found in path
/// - `mod_name`: if `meshes` is not found
pub fn parse_dar_path(path: impl AsRef<Path>) -> Result<ParsedPath> {
    let path = path.as_ref();
    let paths: Vec<&OsStr> = path.iter().collect();

    let is_1st_person = path.iter().any(|os_str| os_str == OsStr::new("_1stperson"));

    let (dar_root, oar_root) = path
        .iter()
        .position(|os_str| os_str == OsStr::new("DynamicAnimationReplacer"))
        .and_then(|idx| {
            paths.get(0..idx).map(|str_paths| {
                let mut dar = Path::new(&str_paths.join(OsStr::new("/"))).to_path_buf();
                let mut oar = dar.clone();
                dar.push("DynamicAnimationReplacer");
                oar.push("OpenAnimationReplacer");
                (dar, oar)
            })
        })
        .ok_or(ConvertError::NotFoundDarDir)?;

    let mod_name = path
        .iter()
        .position(|os_str| os_str.eq_ignore_ascii_case(OsStr::new("meshes")))
        .and_then(|idx| {
            paths
                .get(idx - 1)
                .and_then(|mod_name| mod_name.to_str().map(str::to_owned))
        });

    let priority = path
        .iter()
        .position(|os_str| os_str == OsStr::new("_CustomConditions"))
        .and_then(|idx| paths.get(idx + 1).and_then(|priority| priority.to_str()))
        .ok_or(ConvertError::NotFoundPriorityDir)?;

    let priority = priority.parse::<i32>().map_err(|_err| priority.into());

    let remain_dir = path
        .iter()
        .position(|os_str| os_str == OsStr::new("_CustomConditions"))
        .and_then(|idx| {
            paths.get(idx + 2..paths.len() - 1).and_then(|str_paths| {
                let string = str_paths.join(OsStr::new("/"));
                match string.is_empty() {
                    true => None,
                    false => Some(PathBuf::from(string)),
                }
            })
        });

    Ok(ParsedPath {
        dar_root,
        oar_root,
        is_1st_person,
        mod_name,
        priority,
        remain_dir,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_dar_path_1st_person() -> Result<()> {
        let path = Path::new("../ModName/Meshes/actors/character/_1stperson/animations/DynamicAnimationReplacer/_CustomConditions/8107000/_conditions.txt");
        let result = parse_dar_path(path);

        assert!(result.is_ok());
        let ParsedPath {
            dar_root,
            oar_root,
            is_1st_person,
            mod_name,
            priority,
            remain_dir,
        } = result?;

        assert_eq!(
            dar_root,
            PathBuf::from(
                "../ModName/Meshes/actors/character/_1stperson/animations/DynamicAnimationReplacer"
            )
        );
        assert_eq!(
            oar_root,
            PathBuf::from(
                "../ModName/Meshes/actors/character/_1stperson/animations/OpenAnimationReplacer"
            )
        );
        assert_eq!(is_1st_person, true);
        assert_eq!(mod_name, Some("ModName".to_string()));
        assert_eq!(priority, Ok(8_107_000));
        assert_eq!(remain_dir, None);
        Ok(())
    }

    #[test]
    fn test_parse_dar_path_3rd_person() -> Result<()> {
        let path = Path::new("../ModName/meshes/actors/character/animations/DynamicAnimationReplacer/_CustomConditions/8107000/InnerDir/_conditions.txt");
        let result = parse_dar_path(path);

        assert!(result.is_ok());
        let ParsedPath {
            dar_root,
            oar_root,
            is_1st_person,
            mod_name,
            priority,
            remain_dir,
        } = result?;

        assert_eq!(
            dar_root,
            PathBuf::from("../ModName/meshes/actors/character/animations/DynamicAnimationReplacer")
        );
        assert_eq!(
            oar_root,
            PathBuf::from("../ModName/meshes/actors/character/animations/OpenAnimationReplacer")
        );
        assert_eq!(is_1st_person, false);
        assert_eq!(mod_name, Some("ModName".to_string()));
        assert_eq!(priority, Ok(8_107_000));
        assert_eq!(remain_dir, Some(Path::new("InnerDir").to_path_buf()));
        Ok(())
    }

    #[test]
    fn test_parse_dar_path_invalid_utf8() {
        assert!(parse_dar_path("invalid_path").is_err());
    }
}
