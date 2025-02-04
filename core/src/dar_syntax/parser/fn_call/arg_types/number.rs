//! DAR Number type
use super::Stream;
use crate::dar_syntax::{ast::number_literal::NumberLiteral, parser::winnow_wrapper::float::float};
use std::num::ParseIntError;
use winnow::{
    ascii::{dec_int, digit1, hex_digit1, oct_digit1},
    combinator::{alt, fail},
    dispatch,
    error::{
        AddContext, FromExternalError, ParserError,
        StrContext::{self, Expected, Label},
        StrContextValue::{Description, StringLiteral},
    },
    token::take,
    ModalResult, Parser as _,
};

/// Replace a prefixed radix number such as `0x` with Replace with hexadecimal number without prefix.
fn radix_digits<'i, E>(input: &mut Stream<'i>) -> ModalResult<NumberLiteral, E>
where
    E: ParserError<Stream<'i>>
        + AddContext<Stream<'i>, StrContext>
        + FromExternalError<Stream<'i>, ParseIntError>,
{
    dispatch!(take(2_usize);
        "0b" | "0B" => digit1.try_map(|s| usize::from_str_radix(s, 2))
                        .context(Label("digit")).context(Expected(Description("binary"))),
        "0o" | "0O" => oct_digit1.try_map(|s| usize::from_str_radix(s, 8))
                        .context(Label("digit")).context(Expected(Description("octal"))),
        "0d" | "0D" => digit1.try_map(|s: &str| s.parse::<usize>())
                        .context(Label("digit")).context(Expected(Description("decimal"))),
        "0x" | "0X" => hex_digit1.try_map(|s|usize::from_str_radix(s, 16))
                        .context(Label("digit")).context(Expected(Description("hexadecimal"))),
        _ => fail.context(Label("radix prefix"))
                .context(Expected(StringLiteral("0b")))
                .context(Expected(StringLiteral("0o")))
                .context(Expected(StringLiteral("0d")))
                .context(Expected(StringLiteral("0x"))),
    )
    .map(NumberLiteral::Hex)
    .parse_next(input)
}

/// Parse a number(e.g. "0x123", "123", "12.3")
pub fn number<'i, E>(input: &mut Stream<'i>) -> ModalResult<NumberLiteral, E>
where
    E: ParserError<Stream<'i>>
        + AddContext<Stream<'i>, StrContext>
        + FromExternalError<Stream<'i>, ParseIntError>,
{
    alt((
        radix_digits.context(Label("number")),
        float.map(NumberLiteral::Float).context(Label("number")),
        dec_int.map(NumberLiteral::Decimal).context(Label("number")),
        // At this point, if the string `Hi`, etc. is received, the following error report is made.
        fail.context(Label("number"))
            .context(Expected(Description("radix: e.g. `0x007`")))
            .context(Expected(Description("float: e.g. `33.0`")))
            .context(Expected(Description("decimal: e.g. `10`"))),
    ))
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dar_syntax::parser::parse_assert;
    use winnow::error::ContextError;

    #[test]
    fn should_parse_radix_number() {
        parse_assert!(radix_digits("0b1010"), NumberLiteral::Hex(10));
        parse_assert!(radix_digits("0B1010"), NumberLiteral::Hex(10));
        parse_assert!(radix_digits("0o37"), NumberLiteral::Hex(31));
        parse_assert!(radix_digits("0O37"), NumberLiteral::Hex(31));
        parse_assert!(radix_digits("0x00000007"), NumberLiteral::Hex(7));
        parse_assert!(radix_digits("0X1A"), NumberLiteral::Hex(26));
    }

    #[test]
    fn should_error_radix_number() {
        assert!(radix_digits::<ContextError>.parse("0z123").is_err());
        assert!(radix_digits::<ContextError>.parse("0x").is_err());
    }

    #[test]
    fn should_parse_number() {
        parse_assert!(number("33"), NumberLiteral::Decimal(33));
        parse_assert!(number("33.0"), NumberLiteral::Float(33.0));
        parse_assert!(number("0x00000007"), NumberLiteral::Hex(0x00000007));
    }
}
