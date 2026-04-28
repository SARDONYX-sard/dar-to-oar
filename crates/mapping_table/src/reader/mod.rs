use winnow::ascii::{multispace0, space1, till_line_ending};
use winnow::combinator::{alt, delimited, fail, preceded, repeat, seq};
use winnow::error::{StrContext, StrContextValue};
use winnow::prelude::*;
use winnow_ext::take_until_ext;

use crate::MappingTable;

pub use winnow_ext::ReadableError;

/// Parse a mapping table string into a [`MappingTable`].
///
/// Each line must be one of:
/// - `key name`
/// - `key`
///
/// Lines may contain trailing comments starting with `//`.
/// Empty lines and comment-only lines are ignored.
///
/// When a name is omitted, it is inferred from the previous
/// section name with a sequential suffix.
///
/// # Note
/// If comment lines are included, table includes `{ "priority": 0, rename_to: "" }` dummy.
///
/// # Errors
///
/// Returns an error if a line cannot be parsed.
pub fn parse_mapping_table(input: &str) -> Result<MappingTable, ReadableError> {
    parse_mapping_table_inner
        .parse(input)
        .map_err(ReadableError::from_parse)
}

fn parse_mapping_table_inner(input: &mut &str) -> ModalResult<MappingTable> {
    let mut result: MappingTable = MappingTable::default();

    let mut last_base_name: Option<&str> = None;
    let mut suffix_counter: usize = 0;

    while !input.is_empty() {
        multispace0.parse_next(input)?;
        line_comments0.parse_next(input)?;

        let (priority, name) = parse_line.parse_next(input)?;

        line_comments0.parse_next(input)?;
        multispace0(input)?;

        match (priority, name) {
            (priority, Some(name)) => {
                suffix_counter = 0;
                last_base_name = Some(name);
                result.insert(priority.to_string(), name.to_string());
            }
            (priority, None) => {
                let Some(base) = last_base_name else {
                    fail.context(StrContext::Label("name"))
                        .context(StrContext::Expected(StrContextValue::Description(
                            "name after previous entry",
                        )))
                        .parse_next(input)?
                };

                suffix_counter += 1;
                result.insert(priority.to_string(), format!("{base}_{suffix_counter}"));
            }
        }
    }

    Ok(result)
}

///
/// Parse a single mapping line.
/// returns (priority, Option<name>)
///
/// # Note
/// priority used as actor base id rename target(e.g. `0001A692`). So we use string.
fn parse_line<'a>(input: &mut &'a str) -> ModalResult<(&'a str, Option<&'a str>)> {
    // 2 patterns:
    // 1. priority + space + name
    // 2. priority only

    alt((
        seq! {
            take_until_ext(1.., alt((" ", "\n", "//"))).context(StrContext::Expected(StrContextValue::Description("priority: str"))),
            _: space1,
            take_until_ext(1.., alt(("\n", "//")))
                .context(StrContext::Expected(StrContextValue::Description("rename_to: &str"))),
        }
        .map(|(priority, name): (&str, &str)| (priority, Some(name.trim()))),
        //
        take_until_ext(1.., alt((" ", "\n", "//"))).context(StrContext::Expected(StrContextValue::Description("priority: str")))
        .map(|p| (p, None)),
    ))
    .context(StrContext::Label("mapping line"))
    .context(StrContext::Expected(StrContextValue::Description(
        "format: `<priority> [name]`",
    )))
    .parse_next(input)
}

/// Comments starting with '//' until newline. 0 or more.
fn line_comments0(input: &mut &str) -> ModalResult<()> {
    let _: () = repeat(0.., line_comment).parse_next(input)?;
    Ok(())
}

/// Comment starting with ';' until newline
fn line_comment<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
    delimited(multispace0, preceded("//", till_line_ending), multispace0)
        .context(StrContext::Label("Comment"))
        .context(StrContext::Expected(StrContextValue::Description(
            "Comment(e.g. `// Any String`)",
        )))
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mapping_table() {
        let input = "
8000000  Combat
8000001
8000001  Base
// This is a line comment
8000002
8000005
8000005  Female
8001000
8001000  Unarmed // Comment after name
8001010
8001010  Sword
8001020
8001020  Sword+Shield


// This is a line comment
// This is a line comment
";

        let result = parse_mapping_table(input).unwrap();

        let expected: MappingTable = [
            ("8000000".to_string(), "Combat".to_string()),
            ("8000001".to_string(), "Base".to_string()),
            ("8000002".to_string(), "Base_1".to_string()),
            ("8000005".to_string(), "Female".to_string()),
            ("8001000".to_string(), "Unarmed".to_string()),
            ("8001010".to_string(), "Sword".to_string()),
            ("8001020".to_string(), "Sword+Shield".to_string()),
        ]
        .into();

        assert_eq!(result, expected);
    }
}
