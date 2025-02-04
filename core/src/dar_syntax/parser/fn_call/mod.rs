//! DAR function
pub mod arg_types;
pub mod ident;

use self::arg_types::parse_fn_arg;
use self::ident::ident;
use super::{winnow_wrapper::delimited_with_multispace0::delimited_multispace0, Stream};
use crate::dar_syntax::ast::fn_args::FnArgs;
use std::num::ParseIntError;
use winnow::{
    ascii::multispace0,
    combinator::{delimited, opt, separated},
    error::{
        AddContext, FromExternalError, ModalResult, ParserError,
        StrContext::{self, Label},
    },
    seq, Parser,
};

/// Parse function call(with arguments)
///
/// # Expected Syntax Examples
/// ```txt
/// ; Pattern1
/// IsActorBase("Skyrim.esm" | 0x00000007)
///
/// ; Pattern2
/// IsActorValueEqualTo(0x00000007, 30)
/// ```
pub fn fn_call<'i, E>(input: &mut Stream<'i>) -> ModalResult<(&'i str, FnArgs<'i>), E>
where
    E: ParserError<Stream<'i>>
        + AddContext<Stream<'i>, StrContext>
        + FromExternalError<Stream<'i>, ParseIntError>,
{
    seq!(
        ident,
        opt(delimited(
            delimited_multispace0("("),
            separated(
                0..,
                delimited(multispace0, parse_fn_arg, multispace0).context(Label("FnArg")),
                ","
            ),
            delimited_multispace0(")"),
        ))
        .map(|args| args.unwrap_or_default())
    )
    .context(Label("Function call"))
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dar_syntax::{
        ast::{fn_arg::FnArg, fn_args::fn_args, number_literal::NumberLiteral},
        parser::parse_assert,
    };

    #[test]
    fn should_parse_fn_call() {
        let input = r#"IsActorValueLessThan(30, 60)"#;
        let expected = (
            "IsActorValueLessThan",
            fn_args![
                FnArg::Number(NumberLiteral::Decimal(30)),
                FnArg::Number(NumberLiteral::Decimal(60)),
            ],
        );

        parse_assert!(fn_call(input), expected);
    }
}
