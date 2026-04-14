//! DAR function identifier
pub(crate) mod fn_kind;

use winnow::{
    Parser,
    error::{
        ModalResult,
        StrContext::{Expected, Label},
        StrContextValue::Description,
    },
    token::take_while,
};

/// Parse identifier
pub fn ident<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
    take_while(1.., |c: char| c.is_alphanumeric() || c == '_')
        .context(Label("Identifier"))
        .context(Expected(Description("Identifier(e.g. `IsActorBase`)")))
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use crate::parse_assert;

    use super::*;

    #[test]
    fn should_parse_ident() {
        parse_assert!(ident("IsActorBase"), "IsActorBase");
    }
}
