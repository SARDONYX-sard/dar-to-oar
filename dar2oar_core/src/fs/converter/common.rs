use crate::condition_parser::parse_dar2oar;
use crate::conditions::ConditionsConfig;
use crate::error::{ConvertError, Result};
use crate::fs::converter::{ConvertOptions, ConvertedReport};
use crate::fs::path_changer::ParsedPath;
use crate::fs::section_writer::{read_file, write_name_space_config, write_section_config};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::fs;

/// Common parts of multi-threaded and single-threaded loop processing.
/// # Performance
/// - Since dir is created when a file is discovered, performance is improved if path.is_dir() is not put in path.
pub async fn convert_inner(
    options: &ConvertOptions,
    path: &Path,
    parsed_path: &ParsedPath,
    is_converted_once: &AtomicBool,
) -> Result<()> {
    let ConvertOptions {
        dar_dir: _,
        oar_dir,
        mod_name,
        author,
        section_table,
        section_1person_table,
        run_parallel: _,
        hide_dar: _,
    } = options;

    let is_1st_person = parsed_path.is_1st_person;
    let parsed_mod_name = mod_name
        .clone()
        .unwrap_or(parsed_path.mod_name.clone().unwrap_or("Unknown".into()));
    let oar_name_space_path = oar_dir
        .as_ref()
        .map(|path| match is_1st_person {
            true => Path::new(path)
                .join("meshes/actors/character/_1stperson/animations/OpenAnimationReplacer"),
            false => {
                Path::new(path).join("meshes/actors/character/animations/OpenAnimationReplacer")
            }
        })
        .unwrap_or(parsed_path.oar_root.clone())
        .join(&parsed_mod_name);

    if path.extension().is_some() {
        tracing::debug!("File: {:?}", path);
        let file_name = path
            .file_name()
            .ok_or_else(|| ConvertError::NotFoundFileName)?
            .to_str()
            .ok_or_else(|| ConvertError::InvalidUtf8)?;

        // files that do not have a priority dir, i.e., files on the same level as the priority dir,
        // are copied to the name space folder location.
        // for this reason, an empty string should be put in the name space folder.
        let priority = &parsed_path.priority.clone().unwrap_or_default();
        let section_name = match is_1st_person {
            true => section_1person_table
                .as_ref()
                .and_then(|table| table.get(priority)),
            false => section_table.as_ref().and_then(|table| table.get(priority)),
        }
        .unwrap_or(priority);

        // e.g. mesh/[..]/ModName/SectionName/
        let section_root = oar_name_space_path.join(section_name);
        fs::create_dir_all(&section_root).await?;
        if file_name == "_conditions.txt" {
            let content = read_file(path).await?;
            tracing::debug!("_conditions.txt Content:\n{}", content);

            let config_json = ConditionsConfig {
                name: section_name.into(),
                priority: priority.parse()?,
                conditions: parse_dar2oar(&content)?,
                ..Default::default()
            };
            write_section_config(section_root, config_json).await?;

            if is_converted_once
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
            {
                write_name_space_config(&oar_name_space_path, &parsed_mod_name, author.as_deref())
                    .await?;
            }
        } else {
            // maybe motion files(.hkx), gender dir
            if let Some(remain) = &parsed_path.remain_dir {
                let non_leaf_dir = section_root.join(remain);
                let file = &non_leaf_dir.join(file_name);
                tracing::debug!("Create dirs: {:?}", &non_leaf_dir);
                fs::create_dir_all(&non_leaf_dir).await?;
                tracing::debug!("Copy with Nest Dir:\nfrom: {path:?}\nto: {file:?}");
                fs::copy(path, file).await?;
            } else {
                let file = section_root.join(file_name);
                tracing::debug!("Copy:\nfrom: {path:?}\nto: {file:?}");
                fs::create_dir_all(&section_root).await?;
                fs::copy(path, file).await?;
            }
        }
    }

    Ok(())
}

pub async fn handle_conversion_results<P: AsRef<Path>>(
    hide_dar: bool,
    dar_namespace: &Option<P>,
    dar_1st_namespace: &Option<P>,
) -> Result<ConvertedReport> {
    async fn hide_dar_dir(dir: &Option<impl AsRef<Path>>) -> Result<()> {
        if let Some(dar_dir) = dir {
            let mut hidden_dar = dar_dir.as_ref().to_path_buf();
            hidden_dar.set_extension(".mohidden");
            fs::rename(dar_dir, hidden_dar).await?;
        }
        Ok(())
    }

    if hide_dar {
        hide_dar_dir(dar_namespace).await?;
        hide_dar_dir(dar_1st_namespace).await?;

        match (dar_namespace, dar_1st_namespace) {
            (Some(_), Some(_)) => Ok(ConvertedReport::Renamed1rdAnd3rdPersonDar),
            (Some(_), None) => Ok(ConvertedReport::Renamed3rdPersonDar),
            (None, Some(_)) => Ok(ConvertedReport::Renamed1rdPersonDar),
            _ => Err(ConvertError::NotFoundDarDir),
        }
    } else {
        Ok(ConvertedReport::Complete)
    }
}
