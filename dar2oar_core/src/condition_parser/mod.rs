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
use crate::dar_syntax::{convert_error, syntax::parse_condition};
use crate::error::{ConvertError, Result};

/// Parse a DAR string and convert it into a vector of [`ConditionSets`] representing an OAR structure.
///
/// This function takes a DAR string as input and parses it into a serializable OAR structure.
/// It returns a [`Result`] containing a vector of [`ConditionSet`] if successful,
/// or a [`ConvertError`] if any parsing or conversion error occurs.
pub fn parse_dar2oar(input: &str) -> Result<Vec<ConditionSet>> {
    let (remain, dar_syn) = match parse_condition(input) {
        Ok(syn) => {
            #[cfg(feature = "tracing")]
            tracing::debug!("Input => Parsed DAR:\n{:#?}", syn);
            syn
        }
        Err(err) => {
            let err = match err {
                nom::Err::Incomplete(_) => return Err(ConvertError::IncompleteConversion),
                nom::Err::Error(err) | nom::Err::Failure(err) => err,
            };

            #[cfg(feature = "tracing")]
            tracing::trace!("Entered ConvertError::InvalidDarSyntax");
            return Err(ConvertError::InvalidDarSyntax(convert_error(input, err)));
        }
    };

    match remain.is_empty() {
        true => {
            let oar = parse_conditions(dar_syn)?;
            #[cfg(feature = "tracing")]
            tracing::debug!("Parsed DAR => Serialized OAR:\n{:#?}", &oar);
            Ok(oar.try_into()?)
        }
        false => {
            #[cfg(feature = "tracing")]
            tracing::trace!("Entered ConvertError::IncompleteParseDar");
            Err(ConvertError::IncompleteParseDar(remain.into()))
        }
    }
}
