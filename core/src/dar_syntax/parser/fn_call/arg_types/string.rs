//! DAR string value type
use super::Stream;
use winnow::{
    ascii::take_escaped,
    combinator::{alt, delimited},
    error::{
        AddContext, ModalResult, ParserError,
        StrContext::{self, Expected, Label},
        StrContextValue::Description,
    },
    token::{one_of, take_while},
    Parser,
};

/// single or double quote string
pub fn string<'i, E>(input: &mut Stream<'i>) -> ModalResult<&'i str, E>
where
    E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>,
{
    let single_quote = take_escaped(
        take_while(1.., |c| c != '\'' && c != '\\'),
        '\\',
        one_of(['\\', '\'']),
    );

    let double_quote = take_escaped(
        take_while(1.., |c| c != '\"' && c != '\\'),
        '\\',
        one_of(['\\', '"']),
    );

    alt((
        delimited('\"', single_quote, '\"'),
        delimited('"', double_quote, '"'),
    ))
    .context(Label("String"))
    .context(Expected(Description(r#"String: e.g. `"Skyrim.esm"`"#)))
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dar_syntax::parser::parse_assert;

    #[test]
    fn should_parse_string() {
        parse_assert!(string(r#""0""#), "0");
        parse_assert!(string(r#""with\"escaped""#), "with\\\"escaped");
    }
}
