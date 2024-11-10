//! DAR one line parser
use super::{fn_call::fn_call, Stream};
use crate::dar_syntax::ast::expression::Expression;
use std::num::ParseIntError;
use winnow::{
    ascii::{multispace0, Caseless},
    combinator::opt,
    error::{
        AddContext, FromExternalError, PResult, ParserError,
        StrContext::{self, Expected},
        StrContextValue::StringLiteral,
    },
    seq, Parser,
};

/// Parse one line DAR Syntax
/// # Expected Syntax examples
/// ```txt
/// NOT IsInCombat()
/// ```
pub fn parse_expression<'i, E>(input: &mut Stream<'i>) -> PResult<Expression<'i>, E>
where
    E: ParserError<Stream<'i>>
        + AddContext<Stream<'i>, StrContext>
        + FromExternalError<Stream<'i>, ParseIntError>,
{
    seq!(
        _: multispace0,
        opt(Caseless("NOT")).context(Expected(StringLiteral("NOT")))
            .map(|not| not.is_some()),
        _: multispace0,
        fn_call,
        _: multispace0,
    )
    .map(|(negated, (fn_name, args))| Expression {
        negated,
        fn_name,
        args,
    })
    .parse_next(input)
}
