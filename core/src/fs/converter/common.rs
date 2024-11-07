//! Common parts for sequential and parallel conversions
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
#[cfg_attr(feature = "tracing",
  tracing::instrument(level = "debug", skip(options, is_converted_once), fields(specified_output = &options.oar_dir))
)]
/// Common parts of parallel & sequential loop processing.
pub(super) async fn convert_inner<P>(
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
        oar_dir: specified_oar_root,
        mod_name,
        author,
        section_table,
        section_1person_table,
        hide_dar,
        ..
    } = options;

    let ParsedPath {
        oar_root,
        is_1st_person,
        mod_name: parsed_mod_name,
        actor_name,
        priority,
        remain_dir,
        esp_dir,
        base_id,
        ..
    } = parsed_path;

    let mut parsed_mod_name = mod_name
        .as_deref()
        .unwrap_or_else(|| parsed_mod_name.as_deref().unwrap_or("Unknown"))
        .to_owned();
    if *is_1st_person {
        parsed_mod_name.push_str("_1st_person");
    };

    // character, falmer, etc.
    let actor_name = actor_name.as_deref().unwrap_or({
        #[cfg(feature = "tracing")]
        tracing::warn!(
            "actor_name could not be inferred from the dir name. Use the default value \"character\"."
        );
        "character"
    });

    // "[..]/ModName/meshes/actors/character/animations/OpenAnimationReplacer"
    let oar_name_space = specified_oar_root
        .as_deref()
        .map_or(oar_root.clone(), |oar_mod_root| match is_1st_person {
            true => Path::new(oar_mod_root).join(format!(
                "meshes/actors/{actor_name}/_1stperson/animations/OpenAnimationReplacer"
            )),
            false => Path::new(oar_mod_root).join(format!(
                "meshes/actors/{actor_name}/animations/OpenAnimationReplacer"
            )),
        })
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
                #[cfg(feature = "tracing")]
                tracing::debug!("Copy with Nest Dir: {:?}", remain.join(file_name));
                let non_leaf_dir = $section_root.join(remain); // e.g. mesh/[...]/male/
                fs::create_dir_all(&non_leaf_dir).await?;
                let _ = fs::copy(path, &non_leaf_dir.join(file_name)).await?;
            } else {
                #[cfg(feature = "tracing")]
                tracing::debug!("Copy: {file_name}");
                fs::create_dir_all(&$section_root).await?;
                let _ = fs::copy(path, $section_root.join(file_name)).await?;
            }
        };
    }

    match priority {
        Ok(priority) => {
            // There are two types of pass patterns in the DAR. Process them here.

            let priority_str = priority.to_string();
            // Examples
            // ActorBase: 0001A692 / Condition pattern: 00111000
            let base_id_or_priority_str = base_id.as_ref().unwrap_or(&priority_str);

            //? # Why do you use base_id instead of priority for section names in ActorBase format?
            // In ActorBase format, priority is always 0. If priority is used as it is, it will be indistinguishable
            // from motions for actors with other IDs. To prevent this, use base_id for the section name in ActorBase format.
            let section_name = match is_1st_person {
                true => section_1person_table
                    .as_ref()
                    .and_then(|table| table.get(base_id_or_priority_str.as_str())),
                false => section_table
                    .as_ref()
                    .and_then(|table| table.get(base_id_or_priority_str.as_str())),
            }
            .unwrap_or(base_id_or_priority_str);

            // e.g. mesh/[..]/OpenAnimationReplacer/ModName/SectionName/
            let section_root = oar_name_space.join(section_name);
            fs::create_dir_all(&section_root).await?;

            // - This block is ActorBase pattern
            if esp_dir.is_some() {
                #[cfg(feature = "tracing")]
                tracing::debug!("This path is ActorBase: {path:?}");

                let esp_dir = esp_dir
                    .as_ref()
                    .ok_or(ConvertError::MissingBaseId(path.display().to_string()))?;
                let base_id = base_id
                    .as_ref()
                    .ok_or(ConvertError::MissingBaseId(path.display().to_string()))?;

                let content = format!("IsActorBase ( \"{esp_dir}\" | 0x{base_id} )");
                #[cfg(feature = "tracing")]
                tracing::debug!(
                    "DAR syntax content auto-generated for ActorBase paths:\n{content}"
                );

                let config_json = ConditionsConfig {
                    name: section_name.into(),
                    priority: *priority,
                    conditions: parse_dar2oar(path, &content)?,
                    ..Default::default()
                };

                if !section_root.join("config.json").exists() {
                    write_section_config(&section_root, config_json).await?;
                }

                // # Ordering validity:
                // Use `AcqRel` to `happened before relationship`(form a memory read/write order between threads) of cas(compare_and_swap),
                // so that other threads read after writing true to memory.
                // - In case of cas failure, use `Relaxed` because the order is unimportant.
                let _ = is_converted_once.compare_exchange(false, true, AcqRel, Relaxed);

                // NOTE: If you call the function only once with this is_converted_once flag,
                // the 1st_person&3person conversion will not work!
                write_name_space_config(&oar_name_space, &parsed_mod_name, author.as_deref())
                    .await?;
            };

            // - This block is Condition pattern
            // If `_condition.txt` exists in the ActorBase path, the `_condition.txt` file will be overwritten by `_config.json`,
            // but this problem is not considered in ActorBase because `_condition.txt` should not exist.
            if file_name == "_conditions.txt" {
                let content = fs::read_to_string(path).await?;
                #[cfg(feature = "tracing")]
                tracing::debug!("{path:?} Content:\n{}", content);

                let config_json = ConditionsConfig {
                    name: section_name.into(),
                    priority: *priority,
                    conditions: parse_dar2oar(path, &content)?,
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
                copy_other_file!(section_root);
            };
        }
        Err(invalid_priority) => {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                r#"Got invalid priority: "{invalid_priority}". DAR expects "DynamicAnimationReplacer/_CustomConditions/<numeric directory name>/". Thus, copy it as a memo."#
            );
            let section_root = oar_name_space.join(invalid_priority);
            // This is a consideration so that if a file is directly under DynamicAnimationReplacer,
            // it will be copied in the same way directly under OpenAnimationReplacer.
            let section_root = match section_root.extension().is_some() {
                true => oar_name_space,
                false => section_root,
            };
            copy_other_file!(section_root);
        }
    };

    if *hide_dar && is_contain_oar(path).is_none() {
        hide_path(path).await?;
    };
    Ok(())
}

/// Asynchronously hide a path by renaming it with a ".mohidden" extension.
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

    #[cfg(feature = "tracing")]
    tracing::debug!("Rename:\nfrom: {path:?}\nto: {hidden_path:?}");
    fs::rename(path, &hidden_path).await?;
    Ok(())
}

/// Handle conversion results based on whether any conversions occurred.
#[inline]
pub(super) const fn handle_conversion_results(is_converted_once: bool) -> Result<()> {
    match is_converted_once {
        true => Ok(()),
        false => Err(ConvertError::NeverConverted),
    }
}

/// Find `DynamicAnimationReplacer` string in a argument
#[inline]
pub(super) fn is_contain_dar(path: impl AsRef<Path>) -> Option<usize> {
    path.as_ref()
        .iter()
        .position(|os_str| os_str.eq_ignore_ascii_case("DynamicAnimationReplacer"))
}
