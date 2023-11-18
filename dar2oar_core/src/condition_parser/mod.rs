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

pub fn parse_dar2oar(input: &str) -> Result<Vec<ConditionSet>> {
    let (remain, dar_syn) = match parse_condition(input) {
        Ok(syn) => {
            tracing::debug!("Input => Parsed DAR:\n{:?}", syn);
            syn
        }
        Err(err) => {
            let err = match err {
                nom::Err::Incomplete(_) => return Err(ConvertError::IncompleteConversion),
                nom::Err::Error(err) => err,
                nom::Err::Failure(err) => err,
            };
            return Err(ConvertError::InvalidDarSyntax(convert_error(input, err)));
        }
    };

    match remain.is_empty() {
        true => {
            let oar = parse_conditions(dar_syn)?;
            tracing::debug!("Parsed DAR => Serialized OAR:\n{:?}", &oar);
            Ok(oar.try_into()?)
        }
        false => Err(ConvertError::IncompleteParseDar(remain.into())),
    }
}
