use anyhow::{Context as _, Result};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// - Ok: oar root path, is_1st_person_dir, mod name, priority, non leaf remain(Path excluding file from priority to the end)
/// - Err: anyhow Error
type DarPathsResult = Result<(
    PathBuf,
    PathBuf,
    bool,
    Option<String>,
    Option<String>,
    Option<PathBuf>,
)>;

/// # Returns
/// oar root path, is_1st_person_dir, mod name, priority, non leaf remain(Path excluding file from priority to the end)
///
/// # Errors
/// - If `DynamicAnimationReplacer` is not found in path
///
/// # When does the return value return None?
/// Each of the following will be None if not found in path
/// - mod_name: if `meshes` is not found
/// - priority: if `_CustomConditions` is not found
///
/// ## Format
/// # OAR:([]: optional, <>: variable)
/// - "\<ABS or related ParentDir\>/\<ModName\>/meshes/actors/character/\[_1stperson\]/animations/OptionAnimationReplacer/\<NameSpace\>/\<EachSectionName\>"
///
/// # DAR:
/// - "\<ABS or related ParentDir\>/\<ModName\>/_1stperson/character/animations/DynamicAnimationReplacer/_CustomConditions/\<priority\>/_conditions.txt"
pub fn parse_dar_path(path: impl AsRef<Path>, dar_dirname: Option<&str>) -> DarPathsResult {
    let path = path.as_ref();
    let paths: Vec<&OsStr> = path.iter().collect();

    let is_1st_person = path.iter().any(|os_str| os_str == OsStr::new("_1stperson"));

    let (dar_root, oar_root) = path
        .iter()
        .position(|os_str| os_str == OsStr::new(dar_dirname.unwrap_or("DynamicAnimationReplacer")))
        .and_then(|idx| {
            paths.get(0..idx).map(|path| {
                let mut dar = path.join(OsStr::new("/"));
                let mut oar = dar.clone();
                dar.push("/");
                dar.push(dar_dirname.unwrap_or("DynamicAnimationReplacer"));
                oar.push("/OpenAnimationReplacer");
                (Path::new(&dar).to_path_buf(), Path::new(&oar).to_path_buf())
            })
        })
        .with_context(|| {
            format!(
                "Not found DynamicAnimationReplacer dir. got path: {:?}",
                path
            )
        })?;

    let mod_name = path
        .iter()
        .position(|os_str| os_str == OsStr::new("meshes"))
        .and_then(|idx| {
            paths
                .get(idx - 1)
                .and_then(|path| path.to_str().map(|path| path.to_owned()))
        });

    // The name of the priority dir must be
    // - No extension (i.e., should be dir)
    // - The name of the dir must be a number, with no extension (i.e., it should be dir).
    // Other than the above, set to None.
    let priority = path
        .iter()
        .position(|os_str| os_str == OsStr::new("_CustomConditions"))
        .and_then(|idx| {
            paths
                .get(idx + 1)
                .and_then(|path| path.to_str())
                .and_then(|path| {
                    Path::new(path)
                        .extension()
                        .and(None)
                        .or(path.parse::<i64>().is_ok().then(|| path.to_owned()))
                })
        });

    let non_leaf_remain = path
        .iter()
        .position(|os_str| os_str == OsStr::new("_CustomConditions"))
        .and_then(|idx| {
            paths.get(idx + 2..paths.len() - 1).and_then(|path| {
                let string = path.join(OsStr::new("/"));
                match string.is_empty() {
                    true => None,
                    false => Some(Path::new(&string).to_path_buf()),
                }
            })
        });

    Ok((
        dar_root,
        oar_root,
        is_1st_person,
        mod_name,
        priority,
        non_leaf_remain,
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::{ffi::OsStr, path::PathBuf};

    #[test]
    fn test_parse_dar_path_1st_person() {
        let path = Path::new("../ModName/meshes/actors/character/_1stperson/animations/DynamicAnimationReplacer/_CustomConditions/8107000/_conditions.txt");
        let result = parse_dar_path(path, None);

        assert!(result.is_ok());
        let (dar_root, oar_root, is_1st_person, mod_name, priority, remain) = result.unwrap();

        assert_eq!(
            dar_root,
            PathBuf::from("../ModName/meshes/actors/character/_1stperson/animations/DynamicAnimationReplacer")
        );
        assert_eq!(
            oar_root,
            PathBuf::from(
                "../ModName/meshes/actors/character/_1stperson/animations/OpenAnimationReplacer"
            )
        );
        assert_eq!(is_1st_person, true);
        assert_eq!(mod_name, Some("ModName".to_string()));
        assert_eq!(priority, Some("8107000".to_string()));
        assert_eq!(remain, None);
    }

    #[test]
    fn test_parse_dar_path_3rd_person() {
        let path = Path::new("../ModName/meshes/actors/character/animations/DynamicAnimationReplacer.mohidden/_CustomConditions/8107000/InnerDir/_conditions.txt");
        let result = parse_dar_path(path, Some("DynamicAnimationReplacer.mohidden"));

        assert!(result.is_ok());
        let (dar_root, oar_root, is_1st_person, mod_name, priority, remain) = result.unwrap();

        assert_eq!(
            dar_root,
            PathBuf::from("../ModName/meshes/actors/character/animations/DynamicAnimationReplacer.mohidden")
        );
        assert_eq!(
            oar_root,
            PathBuf::from("../ModName/meshes/actors/character/animations/OpenAnimationReplacer")
        );
        assert_eq!(is_1st_person, false);
        assert_eq!(mod_name, Some("ModName".to_string()));
        assert_eq!(priority, Some("8107000".to_string()));
        assert_eq!(remain, Some(Path::new("InnerDir").to_path_buf()));
    }

    #[test]
    fn test_parse_dar_path_invalid_utf8() {
        // Create a path with invalid UTF-8 characters
        let invalid_path = OsStr::new("invalid_path").to_os_string();
        let path = Path::new(&invalid_path);
        let result = parse_dar_path(path, None);
        assert!(result.is_err());
    }
}
