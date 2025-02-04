// SPDX-License-Identifier: MIT
//! This code is a fork of winnow's `float`.(Decimal point `. ` without a decimal point.)
//!
//! Winnow's standard `float` does not error if there is no decimal point. So it is impossible to distinguish between a decimal point and a floating point.
//!
//! # Ref
//! - [MIT License](https://github.com/winnow-rs/winnow/blob/v0.7.1/LICENSE-MIT)
//! - [Code](https://github.com/winnow-rs/winnow/blob/v0.7.1/src/ascii/mod.rs#L1465)
use winnow::{
    ascii::{digit1, Caseless},
    combinator::{dispatch, empty, opt, peek, trace},
    error::ParserError,
    prelude::*,
    stream::{AsBStr, AsChar, Compare, ParseSlice, Stream, StreamIsPartial},
    token::{any, one_of},
};

/// Parse float(Decimal point `. ` without a decimal point.)
///
/// Winnow's standard `float` does not error if there is no decimal point. So it is impossible to distinguish between a decimal point and a floating point.
pub fn float<Input, Output, Error>(input: &mut Input) -> Result<Output, Error>
where
    Input: StreamIsPartial + Stream + Compare<Caseless<&'static str>> + Compare<char> + AsBStr,
    <Input as Stream>::Slice: ParseSlice<Output>,
    <Input as Stream>::Token: AsChar + Clone,
    <Input as Stream>::IterOffsets: Clone,
    Error: ParserError<Input>,
{
    trace("float", move |input: &mut Input| {
        let s = take_float_or_exceptions(input)?;
        s.parse_slice()
            .ok_or_else(|| ParserError::from_input(input))
    })
    .parse_next(input)
}

/// Infinity, NaN or float
#[allow(clippy::trait_duplication_in_bounds)] // HACK: clippy 1.64.0 bug
fn take_float_or_exceptions<I, E: ParserError<I>>(input: &mut I) -> Result<<I as Stream>::Slice, E>
where
    I: StreamIsPartial,
    I: Stream,
    I: Compare<Caseless<&'static str>>,
    I: Compare<char>,
    <I as Stream>::Token: AsChar + Clone,
    <I as Stream>::IterOffsets: Clone,
    I: AsBStr,
{
    dispatch! {opt(peek(any).map(AsChar::as_char));
        Some('N' | 'n') => Caseless("nan").void(),
        Some('+' | '-') => (any, take_unsigned_float_or_exceptions).void(),
        _ => take_unsigned_float_or_exceptions,
    }
    .take()
    .parse_next(input)
}

/// float
#[allow(clippy::trait_duplication_in_bounds)] // HACK: clippy 1.64.0 bug
fn take_unsigned_float_or_exceptions<I, E: ParserError<I>>(input: &mut I) -> Result<(), E>
where
    I: StreamIsPartial,
    I: Stream,
    I: Compare<Caseless<&'static str>>,
    I: Compare<char>,
    <I as Stream>::Token: AsChar + Clone,
    <I as Stream>::IterOffsets: Clone,
    I: AsBStr,
{
    dispatch! {opt(peek(any).map(AsChar::as_char));
        Some('I' | 'i') => (Caseless("inf"), opt(Caseless("inity"))).void(),
        Some('.') => ('.', digit1, take_exp).void(),
        _ => (digit1, ('.', digit1), take_exp).void(), // NOTE: Make `.' required to distinguish it from the decimal point.
    }
    .parse_next(input)
}

/// Parse exponential
#[allow(clippy::trait_duplication_in_bounds)] // HACK: clippy 1.64.0 bug
fn take_exp<I, E: ParserError<I>>(input: &mut I) -> Result<(), E>
where
    I: StreamIsPartial,
    I: Stream,
    I: Compare<char>,
    <I as Stream>::Token: AsChar + Clone,
    <I as Stream>::IterOffsets: Clone,
    I: AsBStr,
{
    dispatch! {opt(peek(any).map(AsChar::as_char));
        Some('E' | 'e') => (one_of(['e', 'E']), opt(one_of(['+', '-'])), digit1).void(),
        _ => empty,
    }
    .parse_next(input)
}
