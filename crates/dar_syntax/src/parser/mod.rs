//! DAR parser
mod comment;
mod expression;
mod function;
mod op;

use self::{
    comment::line_comments0,
    expression::parse_expression,
    op::{Operator, parse_operator},
};

use crate::ast::Dar;

use winnow::{
    Parser,
    ascii::multispace0,
    combinator::{eof, opt},
    error::{
        ModalResult,
        StrContext::{Expected, Label},
        StrContextValue::Description,
    },
    seq,
};

/// Parse DAR syntax.
///
/// # Errors
/// - Invalid as DAR Syntax
pub(crate) fn parse_dar<'i>(input: &mut &'i str) -> ModalResult<Dar<'i>> {
    let mut top_conditions = Dar::And(Vec::new());
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

        line_comments0.parse_next(input)?;
        let (expr, operator) = seq!(parse_expression, opt(parse_operator)).parse_next(input)?;
        line_comments0.parse_next(input)?;
        let _ = multispace0(input)?;

        #[cfg(feature = "tracing")]
        tracing::trace!("expr: {expr:#?}");
        if let Some(operator) = operator {
            match operator {
                Operator::And => {
                    if is_in_or_stmt {
                        or_vec.push(Dar::Exp(expr));

                        if top_conditions
                            .push(Dar::Or(core::mem::take(&mut or_vec)))
                            .is_err()
                        {
                            winnow::combinator::fail
                                .context(Expected(Description("Top Expression cannot push.")))
                                .parse_next(input)?;
                        }

                        is_in_or_stmt = false;
                    } else {
                        if top_conditions.push(Dar::Exp(expr)).is_err() {
                            winnow::combinator::fail
                                .context(Expected(Description("Top Expression cannot push.")))
                                .parse_next(input)?;
                        }
                    }
                }
                Operator::Or => {
                    or_vec.push(Dar::Exp(expr));
                    is_in_or_stmt = true;
                }
            };

            // To support tailing `OR` or `AND` statement.
            if input.is_empty() {
                if is_in_or_stmt
                    && top_conditions
                        .push(Dar::Or(core::mem::take(&mut or_vec)))
                        .is_err()
                {
                    winnow::combinator::fail
                        .context(Expected(Description("Top Expression cannot push.")))
                        .parse_next(input)?;
                }
                break;
            }
        } else {
            if is_in_or_stmt {
                or_vec.push(Dar::Exp(expr));

                if top_conditions.push(Dar::Or(or_vec)).is_err() {
                    winnow::combinator::fail
                        .context(Expected(Description("Top Expression cannot push.")))
                        .parse_next(input)?;
                }
            } else {
                if top_conditions.push(Dar::Exp(expr)).is_err() {
                    winnow::combinator::fail
                        .context(Expected(Description("Top Expression cannot push.")))
                        .parse_next(input)?;
                }
            }

            eof.context(Label("End of file"))
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
