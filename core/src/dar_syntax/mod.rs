//! DAR syntax parser & error handling
pub mod ast;
pub mod errors;
mod parser;

pub use self::ast::{
    condition::Condition, expression::Expression, fn_arg::FnArg, fn_args::FnArgs,
    number_literal::NumberLiteral,
};
pub use self::parser::parse_dar_syntax;
