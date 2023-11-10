mod actor;
mod compare;
mod conditions;
mod dar_interface;
mod equip;
mod faction;
mod has;
mod macros;

use self::conditions::parse_conditions;
use crate::conditions::ConditionSet;
use crate::dar_syntax::{convert_error, syntax::parse_condition};
use anyhow::bail;

pub fn parse_dar2oar(input: &str) -> anyhow::Result<Vec<ConditionSet>> {
    let (remain, dar_syn) = match parse_condition(input) {
        Ok(syn) => {
            tracing::debug!("Input => Parsed DAR:\n{:?}", syn);
            syn
        }
        Err(err) => {
            let err = match err {
                nom::Err::Incomplete(_) => bail!("Error Incomplete"),
                nom::Err::Error(err) => err,
                nom::Err::Failure(err) => err,
            };
            bail!(convert_error(input, err));
        }
    };

    if !remain.is_empty() {
        bail!("DAR syntax error. Unconverted this.:\n{}", remain);
    }

    let oar = parse_conditions(dar_syn)?;
    tracing::debug!("Parsed DAR => Serializable OAR:\n{:?}", oar);
    Ok(oar.try_into()?)
}
