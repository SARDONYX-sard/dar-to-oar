use winnow::ascii::{hex_digit1, space0};
use winnow::combinator::{repeat, seq};
use winnow::prelude::*;
use winnow::token::rest;

/// Parse `123 - name` or `1A2B - name` style directory names.
///
/// Expected (under DAR):
///   .../DynamicAnimationReplacer/<container>/123 - foo/
///   .../DynamicAnimationReplacer/<container>/1A2B - bar/
///
/// Returns `(priority, name)` if successful.
///
/// # Errors
///
/// Returns `None` if the input does not match the expected pattern.
pub(super) fn parse_dir_pattern(input: &str) -> Option<(&str, &str)> {
    parse_dir_pattern_inner.parse(input).ok()
}

fn parse_dir_pattern_inner<'a>(input: &mut &'a str) -> ModalResult<(&'a str, &'a str)> {
    let (priority_str, rename_to) = seq! {
        // hex or decimal (ASCII hex digits)
        hex_digit1,
        _: space0,
        _: repeat::<_, _,  (), _, _>(0.., "-"),
        _: space0,
        rest
    }
    .parse_next(input)?;

    Ok((priority_str, rename_to))
}

/// Remove ASCII digits from a string.
pub(super) fn strip_numbers(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_ascii_digit())
        .collect::<String>()
}
