mod condition_parser;
mod conditions;
mod dar_syntax;
mod values;
mod write_file;

use self::conditions::ConditionSet;
pub use write_file::convert_dar_to_oar;

pub(self) fn parse_dar2oar(input: &str) -> Vec<ConditionSet> {
    let (remain, dar_syn) = match crate::converter::dar_syntax::syntax::parse_condition(input) {
        Ok(syn) => syn,
        Err(err) => panic!("{}", err),
    };
    dbg!(remain);
    self::condition_parser::parse_conditions(dar_syn)
        .unwrap()
        .try_into()
        .unwrap()
}
