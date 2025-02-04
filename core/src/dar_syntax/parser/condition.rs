//! DAR root parser
use super::Stream;
use crate::dar_syntax::{
    ast::{condition::Condition, op::Operator},
    parser::{comment::line_comments0, expression::parse_expression, op::parse_operator},
};
use std::{mem, num::ParseIntError};
use winnow::{
    ascii::multispace0,
    combinator::{eof, opt},
    error::{
        AddContext, FromExternalError, ModalResult, ParserError,
        StrContext::{self, Expected, Label},
        StrContextValue::Description,
    },
    seq, Parser,
};

/// Parse DAR syntax
pub fn parse_condition<'i, E>(input: &mut Stream<'i>) -> ModalResult<Condition<'i>, E>
where
    E: ParserError<Stream<'i>>
        + AddContext<Stream<'i>, StrContext>
        + FromExternalError<Stream<'i>, ParseIntError>,
{
    let mut top_conditions = Condition::And(Vec::new());
    let mut or_vec = Vec::new();
    let mut is_in_or_stmt = false;

    loop {
        #[cfg(feature = "tracing")]
        tracing::trace!("top_conditions = {top_conditions:#?},\nor_vec = {or_vec:#?}");

        let _ = multispace0(input)?;
        // Dealing with cases where nothing is written in _condition.txt
        if input.is_empty() {
            break;
        }

        let _ = line_comments0(input)?;
        let (expr, operator) = seq!(parse_expression, opt(parse_operator)).parse_next(input)?;
        let _ = line_comments0(input)?;
        let _ = multispace0(input)?;

        #[cfg(feature = "tracing")]
        tracing::trace!("expr: {expr:#?}");
        if let Some(operator) = operator {
            match operator {
                Operator::And => {
                    if is_in_or_stmt {
                        or_vec.push(Condition::Exp(expr));
                        top_conditions.push(Condition::Or(mem::take(&mut or_vec)));
                        is_in_or_stmt = false;
                    } else {
                        top_conditions.push(Condition::Exp(expr));
                    }
                }
                Operator::Or => {
                    or_vec.push(Condition::Exp(expr));
                    is_in_or_stmt = true;
                }
            };

            // To support tailing `OR` or `AND` statement.
            if input.is_empty() {
                if is_in_or_stmt {
                    top_conditions.push(Condition::Or(mem::take(&mut or_vec)));
                }
                break;
            }
        } else {
            match is_in_or_stmt {
                true => {
                    or_vec.push(Condition::Exp(expr));
                    top_conditions.push(Condition::Or(or_vec));
                }
                false => top_conditions.push(Condition::Exp(expr)),
            }

            let _ = eof
                .context(Label("End of file"))
                .context(Expected(Description("end of file")))
                .context(Expected(Description("Tailing op: `OR` or `AND`")))
                .context(Expected(Description(
                    "Conditional statement(if it has op): e.g. `NOT IsInCombat() AND`",
                )))
                .parse_next(input)?;
            break;
        }
    }

    Ok(top_conditions)
}
