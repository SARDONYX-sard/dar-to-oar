mod actor;
mod compare;
mod conditions;
mod dar_interface;
mod equip;
mod faction;
mod has;
mod macros;

pub use crate::converter::write_file::convert_dar_to_oar;

use self::conditions::parse_conditions;
use crate::converter::conditions::ConditionSet;
use crate::converter::dar_syntax::convert_error;
use std::error::Error;

pub fn parse_dar2oar(input: &str) -> Result<Vec<ConditionSet>, Box<dyn Error>> {
    let (_, dar_syn) = match crate::converter::dar_syntax::syntax::parse_condition(input) {
        Ok(syn) => syn,
        Err(err) => {
            let err = match err {
                nom::Err::Incomplete(_) => todo!(),
                nom::Err::Error(err) => err,
                nom::Err::Failure(_) => todo!(),
            };

            return Err(convert_error(input, err).into());
        }
    };
    Ok(parse_conditions(dar_syn)?.try_into()?)
}
