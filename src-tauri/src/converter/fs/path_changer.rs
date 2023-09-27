use anyhow::{Context as _, Result};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// # Returns
/// oar root path, mod name, priority, non leaf remain(Path excluding file from priority to the end)
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
pub fn parse_dar_path(
    path: impl AsRef<Path>,
) -> Result<(PathBuf, Option<String>, Option<String>, Option<PathBuf>)> {
    let path = path.as_ref();
    let paths: Vec<&OsStr> = path.iter().collect();

    let oar_root = path
        .iter()
        .position(|os_str| os_str == OsStr::new("DynamicAnimationReplacer"))
        .and_then(|idx| {
            paths.get(0..idx).and_then(|path| {
                let mut string = path.join(OsStr::new("/"));
                string.push("/OpenAnimationReplacer");
                Some(Path::new(&string).to_path_buf())
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
                .and_then(|path| path.to_str().and_then(|path| Some(path.to_owned())))
        });

    let priority = path
        .iter()
        .position(|os_str| os_str == OsStr::new("_CustomConditions"))
        .and_then(|idx| {
            paths
                .get(idx + 1)
                .and_then(|path| path.to_str().and_then(|path| Some(path.to_owned())))
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

    Ok((oar_root, mod_name, priority, non_leaf_remain))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::{ffi::OsStr, path::PathBuf};

    #[test]
    fn test_parse_dar_path_1st_person() {
        let path = Path::new("../ModName/meshes/actors/character/_1stperson/animations/DynamicAnimationReplacer/_CustomConditions/8107000/_conditions.txt");
        let result = parse_dar_path(path);

        assert!(result.is_ok());
        let (oar_root, mod_name, priority, remain) = result.unwrap();

        assert_eq!(
            oar_root,
            PathBuf::from(
                "../ModName/meshes/actors/character/_1stperson/animations/OpenAnimationReplacer"
            )
        );
        assert_eq!(mod_name, Some("ModName".to_string()));
        assert_eq!(priority, Some("8107000".to_string()));
        assert_eq!(remain, None);
    }

    #[test]
    fn test_parse_dar_path_3rd_person() {
        let path = Path::new("../ModName/meshes/actors/character/animations/DynamicAnimationReplacer/_CustomConditions/8107000/InnerDir/_conditions.txt");
        let result = parse_dar_path(path);

        assert!(result.is_ok());
        let (oar_root, mod_name, priority, remain) = result.unwrap();

        assert_eq!(
            oar_root,
            PathBuf::from("../ModName/meshes/actors/character/animations/OpenAnimationReplacer")
        );
        assert_eq!(mod_name, Some("ModName".to_string()));
        assert_eq!(priority, Some("8107000".to_string()));
        assert_eq!(remain, Some(Path::new("InnerDir").to_path_buf()));
    }

    #[test]
    fn test_parse_dar_path_invalid_utf8() {
        // Create a path with invalid UTF-8 characters
        let invalid_path = OsStr::new("invalid_path").to_os_string();
        let path = Path::new(&invalid_path);
        let result = parse_dar_path(path);
        assert!(result.is_err());
    }
}