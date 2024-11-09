//! Represents a high-level condition, which can be an AND combination, OR combination, or a leaf expression.
use super::expression::Expression;

/// Represents a high-level condition, which can be an AND combination, OR combination, or a leaf expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Condition<'input> {
    /// Represents an AND combination of multiple conditions.
    And(Vec<Condition<'input>>),
    /// Represents an OR combination of multiple conditions.
    Or(Vec<Condition<'input>>),
    /// Represents a leaf expression within the condition hierarchy.
    Exp(Expression<'input>),
}

impl<'input> Condition<'input> {
    /// push to inner vec
    ///
    /// # panics
    /// If push to [`Self::Exp`]
    pub(crate) fn push(&mut self, expression: Self) {
        match self {
            Condition::And(inner) | Condition::Or(inner) => inner.push(expression),
            Condition::Exp(_) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Expression cannot push");
            }
        }
    }
}
