//! DAR string value type
use winnow::{
    Parser,
    ascii::take_escaped,
    combinator::{alt, delimited},
    error::{
        ModalResult,
        StrContext::{Expected, Label},
        StrContextValue::Description,
    },
    token::{one_of, take_while},
};

/// single or double quote string
pub fn string<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
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
    use crate::parse_assert;

    use super::*;

    #[test]
    fn should_parse_string() {
        parse_assert!(string(r#""0""#), "0");
        parse_assert!(string(r#""with\"escaped""#), "with\\\"escaped");
    }
}
