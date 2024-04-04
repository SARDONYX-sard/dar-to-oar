//! The path of the DAR is analyzed to obtain the information necessary for the conversion.
//!
//! # Format ([]: optional, <>: variable)
//! - Common: "\<ABS or related `ParentDir`\>/\<`ModName`\>/meshes/actors/<character or creature>/\[_1stperson\]/animations/"
//!
//! ### OAR
//! - Common + "`OptionAnimationReplacer`/\<`NameSpace`\>/\<`EachSectionName`\>"
//!
//! ### DAR Condition path format: (Only priority order assignment is taken into account. In other words, actor-based allocation is not considered.)
//! - Common + "`DynamicAnimationReplacer`/`_CustomConditions`/\<priority\>/_conditions.txt"
//!
//! ### DAR `ActorBase` path format:
//! - Common + `DynamicAnimationReplacer/<esp name>/<actor base id>/<animation dirs and files>`

use crate::error::{ConvertError, Result};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// The information necessary for the conversion
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedPath {
    /// `"ModName/meshes/actors/character/animations/DynamicAnimationReplacer"`
    pub dar_root: PathBuf,
    /// `"ModName/meshes/actors/character/animations/OpenAnimationReplacer"`
    pub oar_root: PathBuf,
    /// A path that contains a directory named `_1stperson`?
    pub is_1st_person: bool,
    /// `ModName` of `"ModName/meshes/actors/character"`
    pub mod_name: Option<String>,
    /// `character` of `"ModName/meshes/actors/character"`
    pub actor_name: Option<String>,
    /// Number is the expected priority dir name of the formal DAR,
    /// but returns Err for the Mod creator who leaves a note.
    pub priority: Result<i32, String>,
    /// `male`, `female`, others dir
    pub remain_dir: Option<PathBuf>,

    /// Appears only in the actor_base directory format.
    ///
    /// # Examples.
    /// `Skyrim.esm`.
    pub esp_dir: Option<String>,
    /// Appears only in the actor_base directory format.
    ///
    /// # Examples.
    /// `0001A692`.
    pub base_id: Option<String>,
}

impl Default for ParsedPath {
    fn default() -> Self {
        Self {
            dar_root: Default::default(),
            oar_root: Default::default(),
            is_1st_person: Default::default(),
            mod_name: Default::default(),
            actor_name: Default::default(),
            priority: Err("".into()),
            remain_dir: Default::default(),
            esp_dir: Default::default(),
            base_id: Default::default(),
        }
    }
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

    let dar_pos = path
        .iter()
        .position(|os_str| os_str.eq_ignore_ascii_case(OsStr::new("DynamicAnimationReplacer")))
        .ok_or(ConvertError::NotFoundDarDir)?;

    // ActorBase pattern only
    let esp_dir = paths.get(dar_pos + 1).and_then(|name| {
        let lower_name = name.to_str()?.to_lowercase();
        if lower_name.ends_with(".esm")
            || lower_name.ends_with(".esp")
            || lower_name.ends_with(".esl")
        {
            Some(name.to_str()?.to_string())
        } else {
            None
        }
    });

    let base_id = esp_dir.as_ref().and_then(|_| {
        paths
            .get(dar_pos + 2)
            .and_then(|base_id| Some(base_id.to_str()?.to_string()))
    });

    let is_1st_person = path.iter().any(|os_str| os_str == OsStr::new("_1stperson"));

    // Condition pattern
    let (dar_root, oar_root) = paths
        .get(0..dar_pos)
        .map(|str_paths| {
            let mut dar = Path::new(&str_paths.join(OsStr::new("/"))).to_path_buf();
            let mut oar = dar.clone();
            dar.push("DynamicAnimationReplacer");
            oar.push("OpenAnimationReplacer");
            (dar, oar)
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

    let actor_name = path
        .iter()
        .position(|os_str| os_str.eq_ignore_ascii_case(OsStr::new("actors")))
        .and_then(|idx| {
            paths
                .get(idx + 1)
                .and_then(|name| name.to_str().map(str::to_owned))
        });

    let priority = if esp_dir.is_some() {
        "0"
    } else {
        path.iter()
            .position(|os_str| os_str == OsStr::new("_CustomConditions"))
            .and_then(|idx| paths.get(idx + 1).and_then(|priority| priority.to_str()))
            .ok_or(ConvertError::NotFoundPriorityDir)?
    };

    let priority = priority.parse::<i32>().map_err(|_err| priority.into());

    let before_remain_pos = match esp_dir {
        Some(_) => Some(dar_pos + 3), // e.g. DynamicAnimationReplacer/Skyrim.esm/00AC/male/
        None => path
            .iter()
            .position(|os_str| os_str == OsStr::new("_CustomConditions"))
            .map(|idx| idx + 2), // e.g. DynamicAnimationReplacer/_CustomConditions/8107000/InnerDir/_conditions.txt"
    };

    // male, female, etc dir
    let remain_dir = before_remain_pos.and_then(|idx| {
        paths.get(idx..paths.len() - 1).and_then(|str_paths| {
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
        actor_name,
        priority,
        remain_dir,
        esp_dir,
        base_id,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_dar_path_1st_person() -> Result<()> {
        let path = Path::new("../ModName/Meshes/actors/character/_1stperson/animations/DynamicAnimationReplacer/_CustomConditions/8107000/_conditions.txt");
        let result = parse_dar_path(path);

        assert!(result.is_ok());
        let ParsedPath {
            dar_root,
            oar_root,
            is_1st_person,
            mod_name,
            actor_name,
            priority,
            remain_dir,
            ..
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
        assert_eq!(mod_name, Some("ModName".into()));
        assert_eq!(actor_name, Some("character".into()));
        assert_eq!(priority, Ok(8_107_000));
        assert_eq!(remain_dir, None);
        Ok(())
    }

    #[test]
    fn should_parse_dar_path_3rd_person() -> Result<()> {
        let path = Path::new("../ModName/meshes/actors/falmer/animations/DynamicAnimationReplacer/_CustomConditions/8107000/InnerDir/_conditions.txt");
        let result = parse_dar_path(path);

        assert!(result.is_ok());
        let ParsedPath {
            dar_root,
            oar_root,
            is_1st_person,
            mod_name,
            actor_name,
            priority,
            remain_dir,
            ..
        } = result?;

        assert_eq!(
            dar_root,
            PathBuf::from("../ModName/meshes/actors/falmer/animations/DynamicAnimationReplacer")
        );
        assert_eq!(
            oar_root,
            PathBuf::from("../ModName/meshes/actors/falmer/animations/OpenAnimationReplacer")
        );
        assert_eq!(is_1st_person, false);
        assert_eq!(mod_name, Some("ModName".into()));
        assert_eq!(actor_name, Some("falmer".into()));
        assert_eq!(priority, Ok(8_107_000));
        assert_eq!(remain_dir, Some(Path::new("InnerDir").into()));
        Ok(())
    }

    #[test]
    fn should_error_invalid_utf8() {
        assert!(parse_dar_path("invalid_path").is_err());
    }

    #[test]
    fn should_parse_actor_base_path() -> Result<()> {
        let path = Path::new("../ModName/meshes/actors/character/animations/DynamicAnimationReplacer/Mod.esp/00123456/male/1hm.hkx");
        let result = parse_dar_path(path);
        let parsed_path = result?;

        assert_eq!(
            parsed_path.dar_root,
            PathBuf::from("../ModName/meshes/actors/character/animations/DynamicAnimationReplacer")
        );
        assert_eq!(parsed_path.esp_dir, Some("Mod.esp".into()));
        assert_eq!(parsed_path.base_id, Some("00123456".into()));
        assert_eq!(parsed_path.remain_dir, Some("male".into()));
        Ok(())
    }

    #[test]
    fn should_error_invalid_actor_base_path() {
        // Missing DynamicAnimationReplacer
        let path1 = Path::new("../ModName/meshes/actors/character/animations/Mod.esp/00123456/");
        assert!(parse_dar_path(path1).is_err());

        // Invalid ESP name
        let path2 = Path::new(
            "../ModName/meshes/actors/character/animations/DynamicAnimationReplacer/00123456/",
        );
        assert!(parse_dar_path(path2).is_err());
    }
}
