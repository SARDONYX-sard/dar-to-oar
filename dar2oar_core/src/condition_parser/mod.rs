//! Module to convert a parsed DAR into a serializable OAR structure.
mod actor;
mod compare;
mod conditions;
mod dar_interface;
mod equip;
mod faction;
mod has;
mod macros;

use self::conditions::parse_conditions;
pub use self::dar_interface::ParseError;
use crate::conditions::ConditionSet;
use crate::dar_syntax::parse_dar_syntax;
use crate::error::{ConvertError, Result};

/// Parse a DAR string and convert it into a vector of [`ConditionSets`] representing an OAR structure.
///
/// This function takes a DAR string as input and parses it into a serializable OAR structure.
/// It returns a [`Result`] containing a vector of [`ConditionSet`] if successful,
/// or a [`ConvertError`] if any parsing or conversion error occurs.
pub fn parse_dar2oar(input: &str) -> Result<Vec<ConditionSet>> {
    match parse_dar_syntax(input) {
        Ok(dar_ast) => {
            #[cfg(feature = "tracing")]
            tracing::debug!("Input => Parsed DAR:\n{:#?}", dar_ast);

            let oar = parse_conditions(dar_ast)?;
            #[cfg(feature = "tracing")]
            tracing::debug!("Parsed DAR => Serialized OAR:\n{:#?}", &oar);
            let oar = oar.try_into()?;
            Ok(oar)
        }
        Err(err) => Err(ConvertError::InvalidDarSyntax(err.to_string())),
    }
}
