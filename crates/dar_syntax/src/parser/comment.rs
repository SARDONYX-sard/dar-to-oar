//! DAR comment parser
use winnow::{
    Parser as _,
    ascii::{multispace0, till_line_ending},
    combinator::{delimited, preceded, repeat},
    error::{
        ModalResult,
        StrContext::{Expected, Label},
        StrContextValue::Description,
    },
};

/// Comments starting with ';' until newline. 0 or more.
pub fn line_comments0(input: &mut &str) -> ModalResult<()> {
    let _: () = repeat(0.., line_comment).parse_next(input)?;
    Ok(())
}

/// Comment starting with ';' until newline
fn line_comment<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
    delimited(multispace0, preceded(';', till_line_ending), multispace0)
        .context(Label("Comment"))
        .context(Expected(Description("Comment(e.g. `; Any String`)")))
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_comment() {
        let input = r#"
        ; comment
"#;
        let actual = line_comment.parse(input).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(actual, " comment");
    }
}
