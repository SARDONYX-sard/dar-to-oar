//! DAR comment parser
use super::Stream;
use winnow::{
    ascii::{multispace0, till_line_ending},
    combinator::{delimited, preceded, repeat},
    error::{
        AddContext, PResult, ParserError,
        StrContext::{self, Expected, Label},
        StrContextValue::Description,
    },
    Parser,
};

/// Comments starting with ';' until newline. 0 or more.
pub fn line_comments0<'i, E>(input: &mut Stream<'i>) -> PResult<Vec<Stream<'i>>, E>
where
    E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>,
{
    repeat(0.., line_comment).parse_next(input)
}

/// Comment starting with ';' until newline
fn line_comment<'i, E>(input: &mut Stream<'i>) -> PResult<Stream<'i>, E>
where
    E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>,
{
    delimited(multispace0, preceded(';', till_line_ending), multispace0)
        .context(Label("Comment"))
        .context(Expected(Description("Comment(e.g. `; Any String`)")))
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dar_syntax::parser::parse_assert;

    #[test]
    fn should_parse_comment() {
        let input = r#"
        ; comment
"#;
        parse_assert!(line_comment(input), " comment");
    }
}
