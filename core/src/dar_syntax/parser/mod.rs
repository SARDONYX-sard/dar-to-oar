//! DAR syntax parser
//!
//! # Example
//! ```txt
//! IsActorBase("Skyrim.esm" | 0x00000007) OR
//! ; comment
//! IsPlayerTeammate() AND
//! IsEquippedRightType(3) OR
//! IsEquippedRightType(4)
//! ```
//!
//! # EBNF
//! - A | B: A or B
//! - \[ A \]: A is option
//! - { "," A }: 0 or more repetitions "," A
//!
//! ```ebnf
//! line          = comment | expression
//!
//! comment       = ";" [^"\n]* ;
//! expression    = [ "NOT" ] function ( "AND" | "OR" ) ;
//! argument_list = argument { "," argument } ;
//! argument      = plugin | number ;
//!
//! function      = identifier | identifier "(" argument_list ")" ;
//!
//! identifier    = (ASCII | "_") { ASCII | "_" } ;
//!
//! plugin        = string "|" number ;
//!
//! string        = "\"" [^"\n]* "\"" | "'" [^'\n]* "'" ;
//! number        = decimal | hex | float ;
//!
//! decimal       = ["-"] digit { digit } ;
//! hex           = "0x" hex_digit { hex_digit } | "0X" hex_digit { hex_digit } ;
//! float         = ["-"] digit { digit } "." digit { digit } ;
//! digit         = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
//! hex_digit     = digit | "a" | "b" | "c" | "d" | "e" | "f" | "A" | "B" | "C" | "D" | "E" | "F"  ;
//! ```
pub mod comment;
pub mod condition;
pub mod expression;
pub mod fn_call;
pub mod op;
pub mod winnow_wrapper;

use super::{
    ast::condition::Condition,
    errors::{readable_error::ReadableError, Result},
};
use condition::parse_condition;
use winnow::{error::ContextError, Parser as _};

/// Parsing target.
pub type Stream<'i> = &'i str;

/// Parse DAR syntax.
pub fn parse_dar_syntax(input: Stream<'_>) -> Result<Condition<'_>> {
    let syntax = input;
    Ok(parse_condition::<ContextError>
        .parse(syntax)
        .map_err(|error| ReadableError::from_parse(error))?)
}

#[cfg(test)]
macro_rules! parse_assert {
    ($parser:ident($input:tt), $expected:expr) => {
        match $parser::<winnow::error::ContextError>.parse($input) {
            Ok(actual) => pretty_assertions::assert_eq!(actual, $expected),
            Err(err) => panic!(
                "{}",
                crate::dar_syntax::errors::readable_error::ReadableError::from_parse(err)
            ),
        }
    };
}
#[cfg(test)]
pub(crate) use parse_assert;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dar_syntax::ast::{
        expression::Expression, fn_arg::FnArg, fn_args::fn_args, number_literal::NumberLiteral,
    };
    use pretty_assertions::assert_eq;

    #[test]
    #[cfg_attr(feature = "tracing", quick_tracing::init)]
    fn should_parse_conditions() {
        let input = r#"
IsActorBase("Skyrim.esm" | 0X000007) AND
NOT IsInCombat() AND
NOT IsActorValueLessThan(30, 60)
      "#;

        let expected = Condition::And(vec![
            Condition::Exp(Expression {
                fn_name: "IsActorBase",
                args: fn_args![FnArg::PluginValue {
                    plugin_name: "Skyrim.esm",
                    form_id: NumberLiteral::Hex(0x7),
                }],
                negated: false,
            }),
            Condition::Exp(Expression {
                fn_name: "IsInCombat",
                args: fn_args![],
                negated: true,
            }),
            Condition::Exp(Expression {
                negated: true,
                fn_name: "IsActorValueLessThan",
                args: fn_args![
                    FnArg::Number(NumberLiteral::Decimal(30)),
                    FnArg::Number(NumberLiteral::Decimal(60)),
                ],
            }),
        ]);

        match parse_dar_syntax(input) {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    #[cfg_attr(feature = "tracing", quick_tracing::init)]
    fn should_parse_conditions_with_comments() {
        let input = r#"
            ; This is start of line comment.

      IsActorBase("Skyrim.esm" | 0x00BCDEF7) Or
            ; Parse test only indent function call.

  noT         IsPlayerTeammate aNd
            ; This is a line comment.
            ; This is a line comment.

      IsEquippedRightType(3) OR

            ; This is a line comment.
            IsEquippedRightType(4)

            ; This is end of line comment.
            ; This is end of line comment.

"#;

        let actor = Expression {
            negated: false,
            fn_name: "IsActorBase",
            args: fn_args![FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x00BC_DEF7),
            }],
        };
        let player = Expression {
            negated: true,
            fn_name: "IsPlayerTeammate",
            args: fn_args![],
        };
        let equip_r3 = Expression {
            negated: false,
            fn_name: "IsEquippedRightType",
            args: fn_args![FnArg::Number(NumberLiteral::Decimal(3))],
        };
        let equip_r4 = Expression {
            negated: false,
            fn_name: "IsEquippedRightType",
            args: fn_args![FnArg::Number(NumberLiteral::Decimal(4))],
        };

        let expected = Condition::And(vec![
            Condition::Or(vec![Condition::Exp(actor), Condition::Exp(player)]),
            Condition::Or(vec![Condition::Exp(equip_r3), Condition::Exp(equip_r4)]),
        ]);

        match parse_dar_syntax(input) {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn should_parse_with_space() {
        let input = r#" IsActorBase ( "Skyrim.esm"|0x00000007 ) "#;
        let expected = Condition::And(vec![Condition::Exp(Expression {
            negated: false,
            fn_name: "IsActorBase",
            args: fn_args![FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x0000_0007),
            }],
        })]);

        assert_eq!(parse_condition::<ContextError>.parse(input), Ok(expected));
    }

    #[test]
    fn should_parse_tailing_or() {
        let input = "NOT IsActorBase ( \"Skyrim.esm\" | 0x00000007 )OR";
        let expected = Condition::And(vec![Condition::Or(vec![Condition::Exp(Expression {
            negated: true,
            fn_name: "IsActorBase",
            args: fn_args![FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x0000_0007),
            }],
        })])]);

        assert_eq!(parse_condition::<ContextError>.parse(input), Ok(expected));
    }

    #[test]
    fn should_parse_tailing_and() {
        let input = "NOT IsActorBase ( \"Skyrim.esm\" | 0x00000007 )AND";
        let expected = Condition::And(vec![Condition::Exp(Expression {
            negated: true,
            fn_name: "IsActorBase",
            args: fn_args![FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x0000_0007),
            }],
        })]);

        assert_eq!(parse_condition::<ContextError>.parse(input), Ok(expected));
    }
}
