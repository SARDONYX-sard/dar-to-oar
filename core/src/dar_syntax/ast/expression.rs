//! DAR One line representation
use super::fn_args::FnArgs;

/// DAR One line representation
#[derive(Debug, Clone, PartialEq)]
pub struct Expression<'input> {
    /// not condition
    pub negated: bool,
    /// function name == condition name
    pub fn_name: &'input str,
    /// arguments
    pub args: FnArgs<'input>,
}
