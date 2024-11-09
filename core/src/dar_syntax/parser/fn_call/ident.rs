//! DAR function identifier
use super::Stream;
use winnow::{
    error::{
        AddContext, PResult, ParserError,
        StrContext::{self, Expected, Label},
        StrContextValue::Description,
    },
    token::take_while,
    Parser,
};

/// Parse identifier
pub fn ident<'i, E>(input: &mut Stream<'i>) -> PResult<&'i str, E>
where
    E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>,
{
    take_while(1.., |c: char| c.is_alphanumeric() || c == '_')
        .context(Label("Identifier"))
        .context(Expected(Description("Identifier(e.g. `IsActorBase`)")))
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dar_syntax::parser::parse_assert;

    #[test]
    fn should_parse_ident() {
        parse_assert!(ident("IsActorBase"), "IsActorBase");
    }
}
