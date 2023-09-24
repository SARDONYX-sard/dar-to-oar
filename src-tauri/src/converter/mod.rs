mod condition_parser;
mod conditions;
mod dar_syntax;
mod values;
mod write_file;

use crate::converter::conditions::ConditionSet;
use crate::converter::dar_syntax::convert_error;
pub use crate::converter::write_file::convert_dar_to_oar;
use std::error::Error;

pub(self) fn parse_dar2oar(input: &str) -> Result<Vec<ConditionSet>, Box<dyn Error>> {
    let (remain, dar_syn) = match crate::converter::dar_syntax::syntax::parse_condition(input) {
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
    dbg!(remain);
    Ok(self::condition_parser::parse_conditions(dar_syn)?.try_into()?)
}
