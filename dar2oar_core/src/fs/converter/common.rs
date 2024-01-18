use crate::condition_parser::parse_dar2oar;
use crate::conditions::ConditionsConfig;
use crate::error::{ConvertError, Result};
use crate::fs::converter::parallel::is_contain_oar;
use crate::fs::converter::ConvertOptions;
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
        hide_dar,
    } = options;

    let is_1st_person = parsed_path.is_1st_person;
    let mut parsed_mod_name = mod_name
        .clone()
        .unwrap_or(parsed_path.mod_name.clone().unwrap_or("Unknown".into()));
    if is_1st_person {
        parsed_mod_name.push_str("_1st_person");
    };

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
            tracing::debug!("{path:?} Content:\n{}", content);

            let config_json = ConditionsConfig {
                name: section_name.into(),
                priority: priority.parse()?,
                conditions: parse_dar2oar(&content)?,
                ..Default::default()
            };
            write_section_config(section_root, config_json).await?;

            // # Ordering validity:
            // Use `AcqRel` to `happened before relationship`(form a memory read/write order between threads) of cas(compare_and_swap),
            // so that other threads read after writing true to memory to prevent unnecessary file writing.
            // - In case of cas failure, use `Relaxed` because the order is unimportant.
            let _ = is_converted_once.compare_exchange(
                false,
                true,
                Ordering::AcqRel,
                Ordering::Relaxed,
            );

            // NOTE: If you call the function only once with this is_converted_once flag,
            // the 1st_person&3person conversion will not work!
            write_name_space_config(&oar_name_space_path, &parsed_mod_name, author.as_deref())
                .await?;
        } else {
            // maybe motion files(.hkx), gender dir
            if let Some(remain) = &parsed_path.remain_dir {
                let non_leaf_dir = section_root.join(remain);
                let file = &non_leaf_dir.join(file_name);
                tracing::debug!("Create dirs: {:?}", &non_leaf_dir);
                fs::create_dir_all(&non_leaf_dir).await?;
                tracing::debug!("Copy with Nest Dir:\n- From: {path:?}\n-   To: {file:?}");
                fs::copy(path, file).await?;
            } else {
                let file = section_root.join(file_name);
                tracing::debug!("Copy:\n- From: {path:?}\n-   To: {file:?}");
                fs::create_dir_all(&section_root).await?;
                fs::copy(path, file).await?;
            }
        }

        if *hide_dar && path.is_file() && is_contain_oar(path).is_none() {
            hide_path(path).await?;
        }
    }

    Ok(())
}

async fn hide_path(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    // NOTE: Do not use `set_extension` as it overwrites rather than adds.
    let mut hidden_path = path.display().to_string();
    if path
        .extension()
        .map(|ext| ext.eq_ignore_ascii_case("mohidden"))
        != Some(true)
    {
        hidden_path.push_str(".mohidden");
    };

    tracing::debug!("Rename:\nfrom: {path:?}\nto: {hidden_path:?}");
    fs::rename(path, &hidden_path).await?;
    Ok(())
}

#[inline]
pub(super) fn handle_conversion_results(is_converted_once: bool) -> Result<()> {
    match is_converted_once {
        true => Ok(()),
        false => Err(ConvertError::NeverConverted),
    }
}
