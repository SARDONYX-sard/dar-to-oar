//! Module to convert a parsed DAR into a serializable OAR structure.
mod actor;
mod compare;
mod conditions;
mod dar_interface;
mod equip;
mod errors;
mod faction;
mod has;
mod macros;

use self::conditions::parse_conditions;
pub use self::errors::ParseError;
use crate::conditions::ConditionSet;
use crate::dar_syntax::parse_dar_syntax;
use crate::error::{ConvertError, Result};
use std::path::Path;

/// Parse a DAR string and convert it into a vector of [`ConditionSets`] representing an OAR structure.
///
/// This function takes a DAR string as input and parses it into a serializable OAR structure.
/// It returns a [`Result`] containing a vector of [`ConditionSet`] if successful,
/// or a [`ConvertError`] if any parsing or conversion error occurs.
///
/// # Info
/// Now, `path` is only used in case of errors.
pub fn parse_dar2oar<P>(path: P, input: &'_ str) -> Result<Vec<ConditionSet<'_>>>
where
    P: AsRef<Path>,
{
    let dar_ast = parse_dar_syntax(input).map_err(|err| ConvertError::InvalidDarSyntax {
        path: path.as_ref().to_path_buf(),
        source: err,
    })?;
    #[cfg(feature = "tracing")]
    tracing::debug!("Input => Parsed DAR:\n{:#?}", dar_ast);

    let oar_ast = parse_conditions(dar_ast)?;
    #[cfg(feature = "tracing")]
    tracing::debug!("Parsed DAR => Serialized OAR:\n{:#?}", &oar_ast);
    Ok(oar_ast.try_into()?)
}
