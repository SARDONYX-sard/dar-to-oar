//! Common parts for sequential and parallel conversions
use crate::error::{ConvertError, Result};
use crate::fs::converter::{ConvertOptions, parallel::is_contain_oar};
use crate::fs::mapping_table::MappingTable;
use crate::fs::path_changer::ParsedPath;
use crate::fs::section_writer::{write_name_space_config, write_section_config};
use crate::parser::parse_dar2oar;
use oar_conditions::conditions::ConditionsConfig;
use std::path::Path;
use tokio::fs;

// ─── Public entry point ───────────────────────────────────────────────────────

#[cfg_attr(feature = "tracing",
  tracing::instrument(level = "debug", skip(options), fields(specified_output = &options.oar_dir))
)]
/// Common parts of parallel & sequential loop processing.
pub(super) async fn common_process<P>(
    options: &ConvertOptions,
    path: P,
    parsed_path: &ParsedPath,
) -> Result<()>
where
    P: AsRef<Path> + core::fmt::Debug,
{
    let path = path.as_ref();
    let ConvertOptions {
        oar_dir: specified_oar_root,
        mod_name,
        author,
        description,
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

    let resolved_mod_name = resolve_mod_name(
        mod_name.as_deref(),
        parsed_mod_name.as_deref(),
        *is_1st_person,
    );

    let actor_name = resolve_actor_name(actor_name.as_deref());

    let oar_name_space = build_oar_namespace(
        specified_oar_root.as_deref(),
        oar_root,
        actor_name,
        *is_1st_person,
        &resolved_mod_name,
    );

    let file_name = path
        .file_name()
        .ok_or(ConvertError::NotFoundFileName)?
        .to_str()
        .ok_or(ConvertError::InvalidUtf8)?;

    match priority {
        Ok(priority) => {
            let priority_str = priority.to_string();
            let base_id_or_priority_str = base_id.as_ref().unwrap_or(&priority_str);

            let section_name = resolve_section_name(
                base_id_or_priority_str,
                *is_1st_person,
                section_table.as_ref(),
                section_1person_table.as_ref(),
            );

            let section_root = oar_name_space.join(section_name);
            fs::create_dir_all(&section_root).await?;

            if esp_dir.is_some() {
                process_actor_base(
                    path,
                    esp_dir,
                    base_id,
                    section_name,
                    *priority,
                    &section_root,
                    &oar_name_space,
                    &resolved_mod_name,
                    author.as_deref(),
                    description.as_deref(),
                )
                .await?;
            }

            if file_name.eq_ignore_ascii_case("_conditions.txt") {
                process_conditions(
                    path,
                    section_name,
                    *priority,
                    section_root,
                    &oar_name_space,
                    &resolved_mod_name,
                    author.as_deref(),
                    description.as_deref(),
                )
                .await?;
            } else {
                copy_motion_file(path, file_name, &section_root, remain_dir.as_deref()).await?;
            }
        }
        Err(invalid_priority) => {
            process_invalid_priority(
                path,
                file_name,
                invalid_priority,
                &oar_name_space,
                remain_dir.as_deref(),
            )
            .await?;
        }
    }

    maybe_hide_path(path, *hide_dar).await
}

// ─── Name resolution helpers ──────────────────────────────────────────────────

/// Resolve the final mod name, appending `_1st_person` suffix when needed.
fn resolve_mod_name(explicit: Option<&str>, parsed: Option<&str>, is_1st_person: bool) -> String {
    let mut name = explicit
        .unwrap_or_else(|| parsed.unwrap_or("Unknown"))
        .to_owned();
    if is_1st_person {
        name.push_str("_1st_person");
    }
    name
}

/// Resolve the actor name, falling back to `"character"` with a warning.
fn resolve_actor_name(actor_name: Option<&str>) -> &str {
    actor_name.unwrap_or_else(|| {
        #[cfg(feature = "tracing")]
        tracing::warn!(
            "actor_name could not be inferred from the dir name. \
             Using the default value \"character\"."
        );
        "character"
    })
}

/// Build the OAR namespace path:
/// `[oar_root_or_specified]/meshes/actors/<actor>/[_1stperson/]animations/OpenAnimationReplacer/<mod_name>`
fn build_oar_namespace(
    specified_oar_root: Option<&str>,
    oar_root: &Path,
    actor_name: &str,
    is_1st_person: bool,
    mod_name: &str,
) -> std::path::PathBuf {
    let base = specified_oar_root
        .map(|root| {
            let rel = if is_1st_person {
                format!("meshes/actors/{actor_name}/_1stperson/animations/OpenAnimationReplacer")
            } else {
                format!("meshes/actors/{actor_name}/animations/OpenAnimationReplacer")
            };
            Path::new(root).join(rel)
        })
        .unwrap_or_else(|| oar_root.to_path_buf());

    base.join(mod_name)
}

/// Look up a human-readable section name from the priority/base-id tables.
///
/// Falls back to the raw `base_id_or_priority_str` when no entry exists.
fn resolve_section_name<'a>(
    base_id_or_priority_str: &'a str,
    is_1st_person: bool,
    section_table: Option<&'a MappingTable>,
    section_1person_table: Option<&'a MappingTable>,
) -> &'a str {
    let table = if is_1st_person {
        section_1person_table
    } else {
        section_table
    };
    table
        .and_then(|t| t.get(base_id_or_priority_str))
        .map(|s| s.as_str())
        .unwrap_or(base_id_or_priority_str)
}

// ─── Branch processors ────────────────────────────────────────────────────────

/// Handle the ActorBase path pattern:
/// auto-generates an `IsActorBase(...)` condition and writes `config.json`.
#[allow(clippy::too_many_arguments)]
async fn process_actor_base(
    path: &Path,
    esp_dir: &Option<String>,
    base_id: &Option<String>,
    section_name: &str,
    priority: i32,
    section_root: &Path,
    oar_name_space: &Path,
    mod_name: &str,
    author: Option<&str>,
    description: Option<&str>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    tracing::debug!("This path is ActorBase: {path:?}");

    let esp_dir = esp_dir
        .as_ref()
        .ok_or_else(|| ConvertError::MissingBaseId {
            path: path.to_path_buf(),
        })?;
    let base_id = base_id
        .as_ref()
        .ok_or_else(|| ConvertError::MissingBaseId {
            path: path.to_path_buf(),
        })?;

    let content = format!("IsActorBase ( \"{esp_dir}\" | 0x{base_id} )");
    #[cfg(feature = "tracing")]
    tracing::debug!("DAR syntax content auto-generated for ActorBase paths:\n{content}");

    let config_json = ConditionsConfig {
        name: section_name.into(),
        priority,
        conditions: parse_dar2oar(path, &content)?,
        ..Default::default()
    };

    if !section_root.join("config.json").exists() {
        write_section_config(section_root, config_json).await?;
    }

    write_name_space_config(oar_name_space, mod_name, author, description).await
}

/// Handle the `_conditions.txt` pattern:
/// reads the file, parses DAR syntax, and writes `config.json`.
#[allow(clippy::too_many_arguments)]
async fn process_conditions(
    path: &Path,
    section_name: &str,
    priority: i32,
    section_root: std::path::PathBuf,
    oar_name_space: &Path,
    mod_name: &str,
    author: Option<&str>,
    description: Option<&str>,
) -> Result<()> {
    let content = fs::read_to_string(path).await?;
    #[cfg(feature = "tracing")]
    tracing::debug!("{} Content:\n{content}", path.display());

    let config_json = ConditionsConfig {
        name: section_name.into(),
        priority,
        conditions: parse_dar2oar(path, &content)?,
        ..Default::default()
    };
    write_section_config(section_root, config_json).await?;
    write_name_space_config(oar_name_space, mod_name, author, description).await
}

/// Copy a motion file (`.hkx`, gender dir, etc.) into the section root,
/// preserving any nested remainder directory.
async fn copy_motion_file(
    path: &Path,
    file_name: &str,
    section_root: &Path,
    remain_dir: Option<&Path>,
) -> Result<()> {
    if let Some(remain) = remain_dir {
        #[cfg(feature = "tracing")]
        tracing::debug!("Copy with nested dir: {:?}", remain.join(file_name));
        let dest_dir = section_root.join(remain);
        fs::create_dir_all(&dest_dir).await?;
        fs::copy(path, dest_dir.join(file_name)).await?;
    } else {
        #[cfg(feature = "tracing")]
        tracing::debug!("Copy: {file_name}");
        fs::create_dir_all(section_root).await?;
        fs::copy(path, section_root.join(file_name)).await?;
    }
    Ok(())
}

/// Handle an invalid (non-numeric) priority directory:
/// copies the file as a memo alongside `OpenAnimationReplacer`.
async fn process_invalid_priority(
    path: &Path,
    file_name: &str,
    invalid_priority: &str,
    oar_name_space: &Path,
    remain_dir: Option<&Path>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    tracing::warn!(
        r#"Got invalid priority: "{invalid_priority}". \
           DAR expects "DynamicAnimationReplacer/_CustomConditions/<numeric directory name>/". \
           Copying it as a memo."#
    );

    let section_root = oar_name_space.join(invalid_priority);
    // If the path itself is a file (has an extension), place it directly under OAR root.
    let section_root = if section_root.extension().is_some() {
        oar_name_space.to_path_buf()
    } else {
        section_root
    };

    copy_motion_file(path, file_name, &section_root, remain_dir).await
}

// ─── Post-processing ──────────────────────────────────────────────────────────

/// Conditionally hide `path` by appending `.mohidden`, skipping OAR-internal paths.
async fn maybe_hide_path(path: &Path, hide_dar: bool) -> Result<()> {
    if hide_dar && is_contain_oar(path).is_none() {
        hide_path(path).await?;
    }
    Ok(())
}

/// Rename `path` to `<path>.mohidden` (idempotent: skips already-hidden paths).
async fn hide_path(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    // NOTE: Do not use `set_extension` — it replaces rather than appends.
    let mut hidden_path = path.display().to_string();

    if path
        .extension()
        .map(|ext| ext.eq_ignore_ascii_case("mohidden"))
        != Some(true)
    {
        hidden_path.push_str(".mohidden");
    }
    #[cfg(feature = "tracing")]
    tracing::debug!("Rename:\nfrom: {path:?}\nto: {hidden_path:?}");
    fs::rename(path, &hidden_path).await?;
    Ok(())
}

// ─── Path utilities ───────────────────────────────────────────────────────────

/// Return the index of `DynamicAnimationReplacer` in `path`, if present.
#[inline]
pub(super) fn is_contain_dar(path: impl AsRef<Path>) -> Option<usize> {
    path.as_ref()
        .iter()
        .position(|os_str| os_str.eq_ignore_ascii_case("DynamicAnimationReplacer"))
}
