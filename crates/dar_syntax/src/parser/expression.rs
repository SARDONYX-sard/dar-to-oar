//! DAR one line parser
use crate::{ast::Expression, parser::function::function};
use winnow::{
    Parser,
    ascii::Caseless,
    combinator::opt,
    error::{ModalResult, StrContext::Expected, StrContextValue::StringLiteral},
    seq,
};
use winnow_ext::delimited_multispace0;

/// Parse one line DAR Syntax
/// # Expected Syntax examples
/// ```txt
/// NOT IsInCombat()
/// ```
pub fn parse_expression<'i>(input: &mut &'i str) -> ModalResult<Expression<'i>> {
    seq! {
        Expression {
            negated: delimited_multispace0(parse_not),
            function: function,
        }
    }
    .parse_next(input)
}

fn parse_not(input: &mut &str) -> ModalResult<bool> {
    opt(Caseless("NOT"))
        .context(Expected(StringLiteral("NOT")))
        .map(|not| not.is_some())
        .parse_next(input)
}
