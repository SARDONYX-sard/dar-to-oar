//! DAR tailing operator parser
use super::Stream;
use crate::dar_syntax::ast::op::Operator;
use winnow::{
    ascii::{multispace0, Caseless},
    combinator::{alt, preceded},
    error::{
        AddContext, PResult, ParserError,
        StrContext::{self, Expected, Label},
        StrContextValue::StringLiteral,
    },
    Parser,
};

/// - Expect an AND or OR string.
/// - After AND or OR comes Expression with a line break in between, so the line break is also checked.
pub fn parse_operator<'i, E>(input: &mut Stream<'i>) -> PResult<Operator, E>
where
    E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>,
{
    preceded(
        multispace0,
        alt((
            Caseless("AND").value(Operator::And),
            Caseless("OR").value(Operator::Or),
        )),
    )
    .context(Label("Operator"))
    .context(Expected(StringLiteral("AND")))
    .context(Expected(StringLiteral("OR")))
    .parse_next(input)
}
