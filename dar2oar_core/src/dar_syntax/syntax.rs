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

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while1},
    character::complete::{char, digit1, hex_digit1, multispace0, not_line_ending, one_of, space0},
    combinator::{map, opt},
    error::context,
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, separated_pair},
};
use std::fmt;

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
            } => write!(f, "\"{plugin_name}\" | {form_id}"),
            FnArg::Number(num) => write!(f, "{num}"),
        }
    }
}

/// Hex | Decimal | Float
#[derive(Debug, Clone, PartialEq)]
pub enum NumberLiteral {
    /// e.g. 0x007
    Hex(usize),
    /// e.g. 1
    Decimal(isize),
    /// e.g. 1.0
    Float(f32),
}

impl fmt::Display for NumberLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hex(hex) => write!(f, "0x{hex:x}"),
            Self::Decimal(decimal) => write!(f, "{decimal}"),
            Self::Float(float) => write!(f, "{float}"),
        }
    }
}

/// DAR One line representation
#[derive(Debug, Clone, PartialEq)]
pub struct Expression<'input> {
    /// not condition
    pub negated: bool,
    /// function name == condition name
    pub fn_name: &'input str,
    /// arguments
    pub args: Vec<FnArg<'input>>,
}

/// AND | OR
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    /// AND
    And,
    /// OR
    Or,
}

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
    fn push(&mut self, expression: Condition<'input>) {
        match self {
            Condition::And(inner) | Condition::Or(inner) => inner.push(expression),
            Condition::Exp(_) => panic!("Expression cannot push"),
        }
    }
}

/// Type alias for a result type using [`nom::IResult`], wrapping potential errors with [`nom::error::VerboseError`]
type IResult<'input, I, O, E = nom::error::VerboseError<&'input str>> = nom::IResult<I, O, E>;

use nom::error::ParseError; // To use from_error_kind
/// A macro for returning an error with a specific error kind in the `nom::error::VerboseError` variant.
macro_rules! bail_kind {
    ($input:ident, $kind:ident) => {
        return Err(nom::Err::Error(nom::error::VerboseError::from_error_kind(
            $input,
            nom::error::ErrorKind::$kind,
        )))
    };
}

/// Parses a string literal enclosed in single or double quotes with support for escaping characters.
fn parse_string(input: &str) -> IResult<&str, &str> {
    alt((
        delimited(
            char('\''),
            escaped(take_while1(|c| c != '\'' && c != '\\'), '\\', one_of("\\'")),
            char('\''),
        ),
        delimited(
            char('"'),
            escaped(take_while1(|c| c != '"' && c != '\\'), '\\', one_of("\\\"")),
            char('"'),
        ),
    ))(input)
}

/// NOTE: All octal and binary notations are replaced by hex (the value to be retained is in decimal), and hexadecimal notation is used for notation.
fn parse_radix_number(input: &str) -> IResult<&str, NumberLiteral> {
    let (input, _) = multispace0(input)?;
    let (input, radix) = alt((
        tag("0X"),
        tag("0B"),
        tag("0O"),
        tag("0x"),
        tag("0b"),
        tag("0o"),
    ))(input)?;
    let (input, digits) = hex_digit1(input)?;

    let base = match radix {
        "0x" | "0X" => 16,
        "0b" | "0B" => 2,
        "0o" | "0O" => 8,
        _ => bail_kind!(input, HexDigit),
    };

    let result = usize::from_str_radix(digits, base);

    match result {
        Ok(number) => Ok((input, NumberLiteral::Hex(number))),
        _ => bail_kind!(input, HexDigit),
    }
}

/// Parse decimal number(e.g. "123")
///
/// ```EBNF
/// decimal       = ["-"] digit { digit } ;
/// ```
fn parse_decimal(input: &str) -> IResult<&str, NumberLiteral> {
    let (input, _) = multispace0(input)?;
    let (input, is_negative) = opt(char('-'))(input)?;
    let (input, digits) = digit1(input)?;
    let parsed_number = digits.parse::<isize>();

    match parsed_number {
        Ok(number) => {
            let signed_number = if is_negative.is_some() {
                -number
            } else {
                number
            };
            Ok((input, NumberLiteral::Decimal(signed_number)))
        }
        _ => bail_kind!(input, Digit),
    }
}

/// Parse float number(e.g. "12.3")
fn parse_float(input: &str) -> IResult<&str, NumberLiteral> {
    let (input, _) = multispace0(input)?;
    let (input, is_negative) = opt(char('-'))(input)?;
    let (input, whole_part) = digit1(input)?;
    let (input, dot) = char('.')(input)?;
    let (input, fraction_part) = digit1(input)?;

    let number_str = format!(
        "{}{}{}{}",
        is_negative.unwrap_or(' '),
        whole_part,
        dot,
        fraction_part
    );

    let parsed_number = number_str.trim().parse::<f32>();

    match parsed_number {
        Ok(number) => Ok((input, NumberLiteral::Float(number))),
        _ => bail_kind!(input, Float),
    }
}

/// Parse a number(e.g. "0x123", "123", "12.3")
fn parse_number(input: &str) -> IResult<&str, NumberLiteral> {
    alt((parse_radix_number, parse_float, parse_decimal))(input)
}

/// Parse plugin value(e.g. `"Skyrim.esm" | 0x007`)
fn parse_plugin(input: &str) -> IResult<&str, FnArg<'_>> {
    let (input, (plugin_name, form_id)) = separated_pair(
        preceded(space0, parse_string),
        preceded(space0, tag("|")),
        preceded(space0, parse_number),
    )(input)?;

    Ok((
        input,
        FnArg::PluginValue {
            plugin_name,
            form_id,
        },
    ))
}

/// Parse function arguments.
fn parse_argument(input: &str) -> IResult<&str, FnArg<'_>> {
    alt((parse_plugin, map(parse_number, FnArg::Number)))(input)
}

/// Prase identifier
fn parse_ident(input: &str) -> IResult<&str, &str> {
    context(
        "Expected ident. (Example: IsActorBase)",
        take_while1(|c: char| c.is_alphanumeric() || c == '_'),
    )(input)
}

/// Parse function call(with arguments)
fn parse_fn_call(input: &str) -> IResult<&str, (&str, Vec<FnArg<'_>>)> {
    /// Parse function call with arguments
    #[inline]
    fn with_args(input: &str) -> IResult<&str, Option<Vec<FnArg<'_>>>> {
        let (input, _) = tag("(")(input)?;
        let (input, args) =
            opt(separated_list1(tag(","), preceded(space0, parse_argument)))(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag(")")(input)?;
        Ok((input, args))
    }

    let (input, fn_name) = parse_ident(input)?;
    let (input, _) = multispace0(input)?;
    let (input, args) = opt(with_args)(input)?;

    let args = args.map_or(Vec::new(), Option::unwrap_or_default);
    Ok((input, (fn_name, args)))
}

/// - Expect an AND or OR string.
/// - After AND or OR comes Expression with a line break in between, so the line break is also checked.
fn parse_operator(input: &str) -> IResult<&str, Operator> {
    let (input, _) = multispace0(input)?;
    let (input, operator) = alt((
        map(tag("AND"), |_| Operator::And),
        map(tag("OR"), |_| Operator::Or),
    ))(input)?;
    Ok((input, operator))
}

/// Parse one line DAR Syntax
fn parse_expression(input: &str) -> IResult<&str, Expression> {
    let (input, _) = multispace0(input)?;
    let (input, negate) = opt(tag("NOT"))(input)?;
    let (input, _) = space0(input)?;
    let (input, (function_name, args)) = parse_fn_call(input)?;

    Ok((
        input,
        Expression {
            negated: negate.is_some(),
            fn_name: function_name,
            args,
        },
    ))
}

/// Comments starting with ';' until newline
fn comment(input: &str) -> IResult<&str, &str> {
    let (input, _) = multispace0(input)?;
    let (input, comment) = preceded(char(';'), not_line_ending)(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, comment))
}

/// Parse DAR syntax
pub fn parse_condition(input: &str) -> IResult<&str, Condition<'_>> {
    let mut top_conditions = Condition::And(Vec::new());
    let mut or_vec = Vec::new();
    let mut input_tmp = input;
    let mut is_in_or_stmt = false;

    loop {
        let (input, _) = multispace0(input_tmp)?;
        // Dealing with cases where nothing is written in _condition.txt
        if input.is_empty() {
            break;
        }
        // Skip line comment.
        if let Ok((input, _)) = comment(input) {
            input_tmp = input;
            continue;
        };

        let (input, expr) = parse_expression(input)?;
        let (input, _) = space0(input)?;
        let (input, operator) = opt(parse_operator)(input)?;
        let (input, _) = many0(comment)(input)?;
        let (input, _) = multispace0(input)?;

        if let Some(operator) = operator {
            match operator {
                Operator::And => {
                    if is_in_or_stmt {
                        or_vec.push(Condition::Exp(expr));
                        top_conditions.push(Condition::Or(or_vec.clone()));
                        or_vec.clear();
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

            // NOTE:
            // The "OR" & "AND" at the end is syntactically anathema to DAR in my opinion,
            // but others write it, so it cannot be an error.
            if input.is_empty() {
                if is_in_or_stmt {
                    top_conditions.push(Condition::Or(or_vec.clone()));
                }
                input_tmp = input;
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
            input_tmp = input; // To avoid or.clone, so call here.
            break;
        }
        input_tmp = input;
    }

    Ok((input_tmp, top_conditions))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_hex_number() {
        assert_eq!(parse_radix_number("0x1A"), Ok(("", NumberLiteral::Hex(26))));
    }

    #[test]
    fn test_parse_binary_number() {
        assert_eq!(
            parse_radix_number("0b1010"),
            Ok(("", NumberLiteral::Hex(10)))
        );
    }

    #[test]
    fn test_parse_octal_number() {
        assert_eq!(parse_radix_number("0O37"), Ok(("", NumberLiteral::Hex(31))));
    }

    #[test]
    fn test_parse_invalid_input() {
        assert!(parse_radix_number("0z123").is_err());
    }

    #[test]
    fn test_parse_missing_digits() {
        assert!(parse_radix_number("0x").is_err());
    }

    #[test]
    fn test_parse_conditions() {
        let input = r#"
IsActorBase("Skyrim.esm" | 0X000007) AND
NOT IsInCombat() AND
NOT IsActorValueLessThan(30, 60)
      "#;

        let expected = Condition::And(vec![
            Condition::Exp(Expression {
                fn_name: "IsActorBase",
                args: vec![FnArg::PluginValue {
                    plugin_name: "Skyrim.esm",
                    form_id: NumberLiteral::Hex(0x7),
                }],
                negated: false,
            }),
            Condition::Exp(Expression {
                fn_name: "IsInCombat",
                args: vec![],
                negated: true,
            }),
            Condition::Exp(Expression {
                negated: true,
                fn_name: "IsActorValueLessThan",
                args: vec![
                    FnArg::Number(NumberLiteral::Decimal(30)),
                    FnArg::Number(NumberLiteral::Decimal(60)),
                ],
            }),
        ]);

        assert_eq!(parse_condition(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_conditions_with_comments() {
        let input = r#"
            IsActorBase("Skyrim.esm" | 0x00BCDEF7) OR
            ; Parse test only indent function call.
            IsPlayerTeammate AND
            ; This is a line comment.
            ; This is a line comment.

            IsEquippedRightType(3) OR

            ; This is a line comment.
            IsEquippedRightType(4)

            ; This is end of line comment.

"#;

        let actor = Expression {
            negated: false,
            fn_name: "IsActorBase",
            args: vec![FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x00BC_DEF7),
            }],
        };
        let player = Expression {
            negated: false,
            fn_name: "IsPlayerTeammate",
            args: vec![],
        };
        let equip_r3 = Expression {
            negated: false,
            fn_name: "IsEquippedRightType",
            args: vec![FnArg::Number(NumberLiteral::Decimal(3))],
        };
        let equip_r4 = Expression {
            negated: false,
            fn_name: "IsEquippedRightType",
            args: vec![FnArg::Number(NumberLiteral::Decimal(4))],
        };

        let expected = Condition::And(vec![
            Condition::Or(vec![Condition::Exp(actor), Condition::Exp(player)]),
            Condition::Or(vec![Condition::Exp(equip_r3), Condition::Exp(equip_r4)]),
        ]);

        assert_eq!(parse_condition(input), Ok(("", expected)));
    }

    #[test]
    fn should_parse_with_space() {
        let input = r#" IsActorBase ( "Skyrim.esm"|0x00000007 ) "#;
        let expected = Condition::And(vec![Condition::Exp(Expression {
            negated: false,
            fn_name: "IsActorBase",
            args: vec![FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x0000_0007),
            }],
        })]);
        assert_eq!(parse_condition(input), Ok(("", expected)));
    }

    #[test]
    fn should_parse_tailing_or() {
        let input = "NOT IsActorBase ( \"Skyrim.esm\" | 0x00000007 )OR";
        let expected = Condition::And(vec![Condition::Or(vec![Condition::Exp(Expression {
            negated: true,
            fn_name: "IsActorBase",
            args: vec![FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x0000_0007),
            }],
        })])]);

        assert_eq!(parse_condition(input), Ok(("", expected)));
    }

    #[test]
    fn should_parse_tailing_and() {
        let input = "NOT IsActorBase ( \"Skyrim.esm\" | 0x00000007 )AND";
        let expected = Condition::And(vec![Condition::Exp(Expression {
            negated: true,
            fn_name: "IsActorBase",
            args: vec![FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x0000_0007),
            }],
        })]);

        assert_eq!(parse_condition(input), Ok(("", expected)));
    }
}
