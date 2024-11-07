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
pub fn parse_dar2oar<P>(path: P, input: &str) -> Result<Vec<ConditionSet>>
where
    P: AsRef<Path>,
{
    match parse_dar_syntax(input) {
        Ok(dar_ast) => {
            #[cfg(feature = "tracing")]
            tracing::debug!("Input => Parsed DAR:\n{:#?}", dar_ast);

            let oar_ast = parse_conditions(dar_ast)?;
            #[cfg(feature = "tracing")]
            tracing::debug!("Parsed DAR => Serialized OAR:\n{:#?}", &oar_ast);
            Ok(oar_ast.try_into()?)
        }
        Err(mut err) => Err(ConvertError::InvalidDarSyntax({
            err.title = path.as_ref().to_string_lossy().to_string();
            err.to_string()
        })),
    }
}
