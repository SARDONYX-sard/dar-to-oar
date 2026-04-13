pub mod ast;
mod parser;

pub use winnow_ext::ReadableError;

use crate::parser::condition::parse_dar;

/// Parse DAR syntax.
///
/// # Errors
/// - Invalid as DAR Syntax
pub fn parse_dar_syntax(input: &str) -> Result<ast::Dar<'_>, ReadableError> {
    winnow::Parser::parse(&mut parse_dar, input)
        .map_err(|error| winnow_ext::ReadableError::from_parse(error))
}

#[cfg(test)]
#[macro_export]
macro_rules! parse_assert {
    ($parser:ident($input:tt), $expected:expr) => {
        match $parser.parse($input) {
            Ok(actual) => pretty_assertions::assert_eq!(actual, $expected),
            Err(err) => panic!("{}", winnow_ext::ReadableError::from_parse(err)),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Dar, Expression, Function, HandType};
    use oar_values::{FormID, PluginValue, StaticValue, WeaponType};
    use pretty_assertions::assert_eq;

    #[test]
    #[cfg_attr(feature = "tracing", quick_tracing::init)]
    fn should_parse_conditions() {
        let input = r#"
IsActorBase("Skyrim.esm" | 0X000007) AND
NOT IsInCombat() AND
NOT IsActorValueLessThan(30, 60)
      "#;

        let expected = Dar::And(vec![
            Dar::Exp(Expression {
                function: Function::IsActorBase {
                    actor_base: PluginValue {
                        plugin_name: "Skyrim.esm".into(),
                        form_id: FormID::new("7").unwrap(),
                    },
                },
                negated: false,
            }),
            Dar::Exp(Expression {
                function: Function::IsInCombat,
                negated: true,
            }),
            Dar::Exp(Expression {
                function: Function::IsActorValueLessThan {
                    actor_value: StaticValue { value: 30.0 },
                    value: StaticValue { value: 60.0 },
                },
                negated: true,
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

        let expected = Dar::And(vec![
            Dar::Or(vec![
                Dar::Exp(Expression {
                    function: Function::IsActorBase {
                        actor_base: PluginValue {
                            plugin_name: "Skyrim.esm".into(),
                            form_id: FormID::new("BCDEF7").unwrap(),
                        },
                    },
                    negated: false,
                }),
                Dar::Exp(Expression {
                    function: Function::IsPlayerTeammate,
                    negated: true,
                }),
            ]),
            Dar::Or(vec![
                Dar::Exp(Expression {
                    function: Function::IsEquippedType {
                        weapon_type: WeaponType::WarAxe, // 3.0
                        hand_type: HandType::Right,
                    },
                    negated: false,
                }),
                Dar::Exp(Expression {
                    function: Function::IsEquippedType {
                        weapon_type: WeaponType::Mace, // 4.0
                        hand_type: HandType::Right,
                    },
                    negated: false,
                }),
            ]),
        ]);

        match parse_dar_syntax(input) {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn should_parse_with_space() {
        let input = r#" IsActorBase ( "Skyrim.esm"|0x00000007 ) "#;

        let expected = Dar::And(vec![Dar::Exp(Expression {
            function: Function::IsActorBase {
                actor_base: PluginValue {
                    plugin_name: "Skyrim.esm".into(),
                    form_id: FormID::new("7").unwrap(),
                },
            },
            negated: false,
        })]);

        assert_eq!(parse_dar_syntax(input), Ok(expected));
    }

    #[test]
    fn should_parse_tailing_or() {
        let input = "NOT IsActorBase ( \"Skyrim.esm\" | 0x00000007 )OR";

        let expected = Dar::And(vec![Dar::Or(vec![Dar::Exp(Expression {
            function: Function::IsActorBase {
                actor_base: PluginValue {
                    plugin_name: "Skyrim.esm".into(),
                    form_id: FormID::new("7").unwrap(),
                },
            },
            negated: true,
        })])]);

        assert_eq!(parse_dar_syntax(input), Ok(expected));
    }

    #[test]
    fn should_parse_tailing_and() {
        let input = "NOT IsActorBase ( \"Skyrim.esm\" | 0x00000007 )AND";

        let expected = Dar::And(vec![Dar::Exp(Expression {
            function: Function::IsActorBase {
                actor_base: PluginValue {
                    plugin_name: "Skyrim.esm".into(),
                    form_id: FormID::new("7").unwrap(),
                },
            },
            negated: true,
        })]);

        assert_eq!(parse_dar_syntax(input), Ok(expected));
    }
}
