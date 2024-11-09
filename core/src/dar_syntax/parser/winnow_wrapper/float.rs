// SPDX-License-Identifier: MIT
//! This code is a fork of winnow's `float`.(Decimal point `. ` without a decimal point.)
//!
//! Winnow's standard `float` does not error if there is no decimal point. So it is impossible to distinguish between a decimal point and a floating point.
//!
//! # Ref
//! - [MIT License](https://github.com/winnow-rs/winnow/blob/v0.6.20/LICENSE-MIT)
//! - [Code](https://github.com/winnow-rs/winnow/blob/8674ed2c3f57963a4e96c56616ee0a35f58cd258/src/ascii/mod.rs#L1485)
use winnow::{
    ascii::{digit1, Caseless},
    combinator::{alt, cut_err, opt, trace},
    error::{ErrMode, ErrorKind, ParserError},
    prelude::*,
    stream::{AsBStr, AsChar, Compare, ParseSlice, Stream, StreamIsPartial},
    token::{literal, one_of},
};

/// Parse float(Decimal point `. ` without a decimal point.)
///
/// Winnow's standard `float` does not error if there is no decimal point. So it is impossible to distinguish between a decimal point and a floating point.
pub fn float<Input, Output, Error>(input: &mut Input) -> PResult<Output, Error>
where
    Input: StreamIsPartial + Stream + Compare<Caseless<&'static str>> + Compare<char> + AsBStr,
    <Input as Stream>::Slice: ParseSlice<Output>,
    <Input as Stream>::Token: AsChar + Clone,
    <Input as Stream>::IterOffsets: Clone,
    Error: ParserError<Input>,
{
    trace("float", move |input: &mut Input| {
        let s = recognize_float_or_exceptions(input)?;
        s.parse_slice()
            .ok_or_else(|| ErrMode::from_error_kind(input, ErrorKind::Verify))
    })
    .parse_next(input)
}

/// Infinity, NaN or float
#[allow(clippy::trait_duplication_in_bounds)] // HACK: clippy 1.64.0 bug
#[allow(clippy::type_repetition_in_bounds)]
fn recognize_float_or_exceptions<I, E: ParserError<I>>(
    input: &mut I,
) -> PResult<<I as Stream>::Slice, E>
where
    I: StreamIsPartial,
    I: Stream,
    I: Compare<Caseless<&'static str>>,
    I: Compare<char>,
    <I as Stream>::Token: AsChar + Clone,
    <I as Stream>::IterOffsets: Clone,
    I: AsBStr,
{
    alt((
        recognize_float,
        literal(Caseless("nan")),
        (opt(one_of(['+', '-'])), literal(Caseless("infinity"))).take(),
        (opt(one_of(['+', '-'])), literal(Caseless("inf"))).take(),
    ))
    .parse_next(input)
}

/// float
#[allow(clippy::trait_duplication_in_bounds)] // HACK: clippy 1.64.0 bug
#[allow(clippy::type_repetition_in_bounds)]
fn recognize_float<I, E: ParserError<I>>(input: &mut I) -> PResult<<I as Stream>::Slice, E>
where
    I: StreamIsPartial,
    I: Stream,
    I: Compare<char>,
    <I as Stream>::Token: AsChar + Clone,
    <I as Stream>::IterOffsets: Clone,
    I: AsBStr,
{
    (
        opt(one_of(['+', '-'])),
        alt(((digit1, ('.', opt(digit1))).void(), ('.', digit1).void())),
        opt((one_of(['e', 'E']), opt(one_of(['+', '-'])), cut_err(digit1))),
    )
        .take()
        .parse_next(input)
}
