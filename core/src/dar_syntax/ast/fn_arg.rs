//! DAR Function arguments
use super::number_literal::NumberLiteral;
use core::fmt;

/// DAR Function arguments
/// - Plugin e.g. Skyrim.esm | 0x007
/// - Literal e.g. 1.0
#[derive(Debug, Clone, PartialEq)]
pub enum FnArg<'input> {
    /// e.g. "Skyrim.esm" | 0x007
    PluginValue {
        /// e.g. "Skyrim.esm"
        plugin_name: &'input str,
        /// e.g. 1
        form_id: NumberLiteral,
    },
    /// Just number. (e.g. 1)
    Number(NumberLiteral),
}

impl fmt::Display for FnArg<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FnArg::PluginValue {
                plugin_name,
                form_id,
            } => write!(f, r#""{plugin_name}" | {form_id}"#),
            FnArg::Number(num) => write!(f, "{num}"),
        }
    }
}
