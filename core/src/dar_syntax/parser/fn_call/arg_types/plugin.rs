//! DAR Plugin value type
use super::{number::number, string::string, Stream};
use crate::dar_syntax::ast::fn_arg::FnArg;
use std::num::ParseIntError;
use winnow::{
    ascii::multispace0,
    combinator::{delimited, separated_pair},
    error::{
        AddContext, FromExternalError, PResult, ParserError,
        StrContext::{self, Expected, Label},
        StrContextValue::Description,
    },
    Parser,
};

/// Parse plugin value(e.g. `"Skyrim.esm" | 0x007`)
pub fn parse_plugin<'i, E>(input: &mut Stream<'i>) -> PResult<FnArg<'i>, E>
where
    E: ParserError<Stream<'i>>
        + AddContext<Stream<'i>, StrContext>
        + FromExternalError<Stream<'i>, ParseIntError>,
{
    separated_pair(
        delimited(multispace0, string, multispace0),
        "|",
        delimited(multispace0, number, multispace0),
    )
    .map(|(plugin_name, form_id)| FnArg::PluginValue {
        plugin_name,
        form_id,
    })
    .context(Label("Plugin"))
    .context(Expected(Description(
        r#"Plugin: e.g. `"Skyrim.esm" | 0x007`"#,
    )))
    .parse_next(input)
}
