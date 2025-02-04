//! DAR argument types
pub mod number;
pub mod plugin;
pub mod string;

use self::{number::number, plugin::parse_plugin};
use crate::dar_syntax::{ast::fn_arg::FnArg, parser::Stream};
use std::num::ParseIntError;
use winnow::{
    combinator::{alt, fail},
    error::{
        AddContext, FromExternalError, ModalResult, ParserError,
        StrContext::{self, Expected, Label},
        StrContextValue::{Description, StringLiteral},
    },
    Parser,
};

/// Parse function arguments.
pub fn parse_fn_arg<'i, E>(input: &mut Stream<'i>) -> ModalResult<FnArg<'i>, E>
where
    E: ParserError<Stream<'i>>
        + AddContext<Stream<'i>, StrContext>
        + FromExternalError<Stream<'i>, ParseIntError>,
{
    alt((
        parse_plugin,
        number.map(FnArg::Number),
        fail.context(Label("function argument"))
            .context(Expected(Description("plugin")))
            .context(Expected(StringLiteral("number"))),
    ))
    .parse_next(input)
}
