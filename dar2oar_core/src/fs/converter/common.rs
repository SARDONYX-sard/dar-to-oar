use crate::condition_parser::parse_dar2oar;
use crate::conditions::ConditionsConfig;
use crate::error::{ConvertError, Result};
use crate::fs::converter::{parallel::is_contain_oar, ConvertOptions};
use crate::fs::path_changer::ParsedPath;
use crate::fs::section_writer::{write_name_space_config, write_section_config};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering::AcqRel, Ordering::Relaxed};
use tokio::fs;

// NOTE: Variables in fields do not appear in the log if Option::None.
#[tracing::instrument(level = "debug", skip(options, is_converted_once), fields(specified_output = &options.oar_dir))]
/// Common parts of parallel & sequential loop processing.
pub async fn convert_inner<P>(
    options: &ConvertOptions,
    path: P,
    parsed_path: &ParsedPath,
    is_converted_once: &AtomicBool,
) -> Result<()>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let path = path.as_ref();
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

    let ParsedPath {
        dar_root: _,
        oar_root,
        is_1st_person,
        mod_name: parsed_mod_name,
        priority,
        remain_dir,
    } = parsed_path;

    let mut parsed_mod_name = mod_name
        .as_deref()
        .unwrap_or(parsed_mod_name.as_deref().unwrap_or("Unknown"))
        .to_owned();
    if *is_1st_person {
        parsed_mod_name.push_str("_1st_person");
    };

    let oar_name_space = oar_dir
        .as_deref()
        .map(|oar_mod_root| match is_1st_person {
            true => Path::new(oar_mod_root)
                .join("meshes/actors/character/_1stperson/animations/OpenAnimationReplacer"),
            false => Path::new(oar_mod_root)
                .join("meshes/actors/character/animations/OpenAnimationReplacer"),
        })
        .unwrap_or(oar_root.clone())
        .join(&parsed_mod_name);

    let file_name = path
        .file_name()
        .ok_or(ConvertError::NotFoundFileName)?
        .to_str()
        .ok_or(ConvertError::InvalidUtf8)?;

    /// Copy motion files(.hkx), gender dir or other.
    macro_rules! copy_other_file {
        ($section_root:ident) => {
            if let Some(remain) = remain_dir {
                tracing::debug!("Copy with Nest Dir: {:?}", remain.join(file_name));
                let non_leaf_dir = $section_root.join(remain); // e.g. mesh/[...]/male/
                fs::create_dir_all(&non_leaf_dir).await?;
                fs::copy(path, &non_leaf_dir.join(file_name)).await?;
            } else {
                tracing::debug!("Copy: {file_name}");
                fs::create_dir_all(&$section_root).await?;
                fs::copy(path, $section_root.join(file_name)).await?;
            }
        };
    }

    match priority {
        Ok(priority) => {
            let priority_str = priority.to_string();
            let section_name = match is_1st_person {
                true => section_1person_table
                    .as_ref()
                    .and_then(|table| table.get(&priority_str)),
                false => section_table
                    .as_ref()
                    .and_then(|table| table.get(&priority_str)),
            }
            .unwrap_or(&priority_str);

            // e.g. mesh/[..]/OpenAnimationReplacer/ModName/SectionName/
            let section_root = oar_name_space.join(section_name);
            fs::create_dir_all(&section_root).await?;

            if file_name == "_conditions.txt" {
                let content = fs::read_to_string(path).await?;
                tracing::debug!("{path:?} Content:\n{}", content);

                let config_json = ConditionsConfig {
                    name: section_name.into(),
                    priority: *priority,
                    conditions: parse_dar2oar(&content)?,
                    ..Default::default()
                };
                write_section_config(section_root, config_json).await?;

                // # Ordering validity:
                // Use `AcqRel` to `happened before relationship`(form a memory read/write order between threads) of cas(compare_and_swap),
                // so that other threads read after writing true to memory.
                // - In case of cas failure, use `Relaxed` because the order is unimportant.
                let _ = is_converted_once.compare_exchange(false, true, AcqRel, Relaxed);

                // NOTE: If you call the function only once with this is_converted_once flag,
                // the 1st_person&3person conversion will not work!
                write_name_space_config(&oar_name_space, &parsed_mod_name, author.as_deref())
                    .await?;
            } else {
                copy_other_file!(section_root)
            };
        }
        Err(invalid_priority) => {
            tracing::warn!(
                r#"Got invalid priority: "{}". DAR expects "DynamicAnimationReplacer/_CustomConditions/<numeric directory name>/". Thus, copy it as a memo."#,
                invalid_priority
            );
            let section_root = oar_name_space.join(invalid_priority);
            // This is a consideration so that if a file is directly under DynamicAnimationReplacer,
            // it will be copied in the same way directly under OpenAnimationReplacer.
            let section_root = match section_root.extension().is_some() {
                true => oar_name_space,
                false => section_root,
            };
            copy_other_file!(section_root)
        }
    };

    if *hide_dar && is_contain_oar(path).is_none() {
        hide_path(path).await?
    };
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
