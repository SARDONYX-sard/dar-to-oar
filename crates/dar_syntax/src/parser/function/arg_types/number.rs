//! DAR Number type
use std::borrow::Cow;

use oar_values::{Direction, FormID, StaticValue};
use winnow::{
    ModalResult, Parser as _,
    ascii::{digit1, float, hex_digit1, oct_digit1},
    combinator::{alt, fail},
    dispatch,
    error::{
        StrContext::{Expected, Label},
        StrContextValue::{Description, StringLiteral},
    },
    token::take,
};

pub(crate) fn direction(input: &mut &str) -> ModalResult<Direction> {
    float
        .verify_map(|value: f64| {
            Some(match value {
                0.0..1.0 => Direction::None,
                1.0..2.0 => Direction::Forward,
                2.0..3.0 => Direction::Right,
                3.0..4.0 => Direction::Back,
                4.0..5.0 => Direction::Left,
                _ => return None,
            })
        })
        .context(Label("Direction"))
        .context(Expected(Description("0.0..=4.0")))
        .parse_next(input)
}

/// Parse a number(e.g. "0x123", "123", "12.3")
pub(crate) fn static_value(input: &mut &str) -> ModalResult<StaticValue> {
    alt((
        radix_digits_number.context(Label("number")),
        float::<_, f64, _>
            .map(|value| StaticValue { value })
            .context(Label("number")),
        // At this point, if the string `Hi`, etc. is received, the following error report is made.
        fail.context(Label("number"))
            .context(Expected(Description("radix: e.g. `0x007`")))
            .context(Expected(Description("float: e.g. `33.0`")))
            .context(Expected(Description("decimal: e.g. `10`"))),
    ))
    .parse_next(input)
}

/// Replace a prefixed radix number such as `0x` with Replace with hexadecimal number without prefix.
fn radix_digits_number(input: &mut &str) -> ModalResult<StaticValue> {
    dispatch!(take(2_usize);
        "0b" | "0B" => digit1.try_map(|s| usize::from_str_radix(s, 2))
                        .map(|n: usize| StaticValue{ value: n as f64 })
                        .context(Label("radix")).context(Expected(Description("binary"))),

        "0o" | "0O" => oct_digit1.try_map(|s| usize::from_str_radix(s, 8))
                        .map(|n: usize| StaticValue{ value: n as f64 })
                        .context(Label("radix")).context(Expected(Description("octal"))),

        "0d" | "0D" => digit1.try_map(|s: &str| s.parse::<usize>())
                        .map(|n: usize| StaticValue{ value: n as f64 })
                        .context(Label("radix")).context(Expected(Description("decimal"))),

        "0x" | "0X" => hex_digit1.try_map(|s| usize::from_str_radix(s, 16))
                        .map(|n: usize| StaticValue{ value: n as f64 })
                        .context(Label("radix")).context(Expected(Description("hexadecimal"))),

        _ => fail.context(Label("radix prefix"))
                .context(Expected(StringLiteral("0b")))
                .context(Expected(StringLiteral("0o")))
                .context(Expected(StringLiteral("0d")))
                .context(Expected(StringLiteral("0x"))),
    )
    .parse_next(input)
}

/// Parse a number(e.g. "0x123", "123", "12.3")
pub(crate) fn form_id<'i>(input: &mut &'i str) -> ModalResult<FormID<'i>> {
    alt((
        radix_digits.context(Label("number")),
        float::<_, f64, _>
            .map(|n| unsafe { FormID::new_unchecked(Cow::Owned(format!("{:x}", n as u64))) }) // Safety: already checked valid hex.
            .context(Label("number")),
        // At this point, if the string `Hi`, etc. is received, the following error report is made.
        fail.context(Label("number"))
            .context(Expected(Description("radix: e.g. `0x007`")))
            .context(Expected(Description("float: e.g. `33.0`")))
            .context(Expected(Description("decimal: e.g. `10`"))),
    ))
    .parse_next(input)
}

/// Replace a prefixed radix number such as `0x` with Replace with hexadecimal number without prefix.
fn radix_digits<'i>(input: &mut &'i str) -> ModalResult<FormID<'i>> {
    #[inline]
    fn trim_leading_zeros(s: &str) -> &str {
        let trimmed = s.trim_start_matches('0');
        if trimmed.is_empty() { "0" } else { trimmed }
    }

    dispatch!(take(2_usize);
        "0b" | "0B" => digit1.try_map(|s| usize::from_str_radix(s, 2))
                        .map(|n| unsafe { FormID::new_unchecked(Cow::Owned(format!("{n:x}"))) })
                        .context(Label("radix")).context(Expected(Description("binary"))),

        "0o" | "0O" => oct_digit1.try_map(|s| usize::from_str_radix(s, 8))
                        .map(|n| unsafe { FormID::new_unchecked(Cow::Owned(format!("{n:x}"))) })
                        .context(Label("radix")).context(Expected(Description("octal"))),

        "0d" | "0D" => digit1.try_map(|s: &str| s.parse::<usize>())
                        .map(|n| unsafe { FormID::new_unchecked(Cow::Owned(format!("{n:x}"))) })
                        .context(Label("radix")).context(Expected(Description("decimal"))),

        "0x" | "0X" => hex_digit1.map(|s| unsafe { FormID::new_unchecked(Cow::Borrowed(trim_leading_zeros(s))) }),

        _ => fail.context(Label("radix prefix"))
                .context(Expected(StringLiteral("0b")))
                .context(Expected(StringLiteral("0o")))
                .context(Expected(StringLiteral("0d")))
                .context(Expected(StringLiteral("0x"))),
    )
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use crate::parse_assert;

    use super::*;

    #[test]
    fn should_parse_number() {
        parse_assert!(radix_digits("0b1010"), FormID::new("a").unwrap());
        parse_assert!(radix_digits("0B1010"), FormID::new("a").unwrap());
        parse_assert!(radix_digits("0o37"), FormID::new("1f").unwrap()); // 31
        parse_assert!(radix_digits("0O37"), FormID::new("1f").unwrap()); // 31
        parse_assert!(radix_digits("0x00000007"), FormID::new("7").unwrap());
        parse_assert!(radix_digits("0X1A"), FormID::new("1A").unwrap());

        parse_assert!(static_value("33"), StaticValue { value: 33.0 });
        parse_assert!(static_value("33.0"), StaticValue { value: 33.0 });
        parse_assert!(static_value("0x000000a7"), StaticValue { value: 167.0 });
    }

    #[test]
    fn should_error_radix_number() {
        assert!(radix_digits.parse("0z123").is_err());
        assert!(radix_digits.parse("0x").is_err());
    }
}
