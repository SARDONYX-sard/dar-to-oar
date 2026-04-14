//! DAR tailing operator parser
use winnow::{
    Parser as _,
    ascii::{Caseless, multispace0},
    combinator::{alt, preceded},
    error::{
        ModalResult,
        StrContext::{Expected, Label},
        StrContextValue::StringLiteral,
    },
};

/// AND | OR
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    /// AND
    And,
    /// OR
    Or,
}

/// - Expect an AND or OR string.
/// - After AND or OR comes Expression with a line break in between, so the line break is also checked.
pub fn parse_operator(input: &mut &str) -> ModalResult<Operator> {
    preceded(
        multispace0,
        alt((
            Caseless("AND").value(Operator::And),
            Caseless("OR").value(Operator::Or),
        )),
    )
    .context(Label("Operator"))
    .context(Expected(StringLiteral("AND")))
    .context(Expected(StringLiteral("OR")))
    .parse_next(input)
}
