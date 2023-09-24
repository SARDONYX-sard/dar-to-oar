//! DAR syntax parser
//!
//! # Example
//! ```txt
//! IsActorBase("Skyrim.esm" | 0x00000007) OR
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
//! hex           = "0x" hex_digit { hex_digit } ;
//! float         = ["-"] digit { digit } "." digit { digit } ;
//! digit         = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
//! hex_digit     = digit | "a" | "b" | "c" | "d" | "e" | "f" | "A" | "B" | "C" | "D" | "E" | "F"  ;
//! ```

use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while1},
    character::complete::{char, digit1, hex_digit1, multispace0, one_of, space0},
    combinator::{map, opt},
    error::context,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
};

#[derive(Debug, Clone, PartialEq)]
pub enum FnArg<'a> {
    PluginValue {
        plugin_name: &'a str,
        form_id: NumberLiteral,
    },
    Number(NumberLiteral),
}

impl fmt::Display for FnArg<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FnArg::PluginValue {
                plugin_name,
                form_id,
            } => write!(f, "PluginName: {plugin_name}, formID: {form_id}"),
            FnArg::Number(num) => write!(f, "{num}"),
        }
    }
}

/// Hex | Decimal | Float
#[derive(Debug, Clone, PartialEq)]
pub enum NumberLiteral {
    Hex(usize),
    Decimal(isize),
    Float(f32),
}

impl fmt::Display for NumberLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberLiteral::Hex(hex) => write!(f, "0x{hex:x}"),
            NumberLiteral::Decimal(decimal) => write!(f, "{decimal}"),
            NumberLiteral::Float(float) => write!(f, "{float}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression<'a> {
    /// not condition
    pub negated: bool,
    /// function name == condition name
    pub fn_name: &'a str,
    pub args: Vec<FnArg<'a>>,
}

/// AND | OR
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Condition<'a> {
    And(Vec<Condition<'a>>),
    Or(Vec<Condition<'a>>),
    Exp(Expression<'a>),
}

impl<'a> Condition<'a> {
    /// push to inner vec
    ///
    /// # panics
    /// If push to Self::Exp
    fn push(&mut self, expression: Condition<'a>) {
        match self {
            Condition::And(inner) => inner.push(expression),
            Condition::Or(inner) => inner.push(expression),
            Condition::Exp(_) => panic!("Expression cannot push"),
        }
    }
}

/// IResult wrapped for VerboseError
type IResult<'a, I, O> = nom::IResult<I, O, nom::error::VerboseError<&'a str>>;

use nom::error::ParseError; // To use from_error_kind
macro_rules! bail_kind {
    ($input:ident, $kind:ident) => {
        return Err(nom::Err::Error(nom::error::VerboseError::from_error_kind(
            $input,
            nom::error::ErrorKind::$kind,
        )))
    };
}

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

fn parse_radix_number<'a>(input: &'a str) -> IResult<&str, NumberLiteral> {
    let (input, _) = multispace0(input)?;
    let (input, radix) = alt((tag("0x"), tag("0b"), tag("0o")))(input)?;
    let (input, digits) = hex_digit1(input)?;

    let base = match radix {
        "0x" => 16,
        "0b" => 2,
        "0o" => 8,
        _ => bail_kind!(input, HexDigit),
    };

    let result = usize::from_str_radix(digits, base);

    match result {
        Ok(number) => Ok((input, NumberLiteral::Hex(number))),
        _ => bail_kind!(input, HexDigit),
    }
}

fn parse_decimal<'a>(input: &'a str) -> IResult<&str, NumberLiteral> {
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

fn parse_float<'a>(input: &'a str) -> IResult<&str, NumberLiteral> {
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

fn parse_number<'a>(input: &'a str) -> IResult<&str, NumberLiteral> {
    alt((parse_radix_number, parse_float, parse_decimal))(input)
}

fn parse_plugin<'a>(input: &'a str) -> IResult<&'a str, FnArg<'a>> {
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

fn parse_argument<'a>(input: &'a str) -> IResult<&str, FnArg<'a>> {
    alt((parse_plugin, map(parse_number, |f| FnArg::Number(f))))(input)
}

fn parse_ident(input: &str) -> IResult<&str, &str> {
    context(
        "Expected ident. (Example: IsActorBase)",
        take_while1(|c: char| c.is_alphanumeric() || c == '_'),
    )(input)
}

fn parse_fn_call<'a>(input: &'a str) -> IResult<&'a str, (&'a str, Vec<FnArg<'a>>)> {
    let (input, fn_name) = parse_ident(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, args) = opt(separated_list1(tag(","), preceded(space0, parse_argument)))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(")")(input)?;
    let args = match args {
        Some(args) => args,
        None => Vec::new(),
    };

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

pub fn parse_condition<'a>(input: &'a str) -> IResult<&'a str, Condition<'a>> {
    let mut top_conditions = Condition::And(Vec::new());
    let mut or_vec = Vec::new();
    let mut input_tmp = input;
    let mut is_in_or_stmt = false;

    loop {
        let (input, _) = multispace0(input_tmp)?;
        let (input, expr) = parse_expression(input)?;
        let (input, _) = multispace0(input)?;
        let (mut input, operator) = opt(parse_operator)(input)?;
        if operator.is_some() {
            let (inp, _) = space0(input)?;
            let (inp, _) = preceded(opt(char('\r')), char('\n'))(inp)?;
            input = inp;
        }

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
        } else {
            match is_in_or_stmt {
                true => {
                    or_vec.push(Condition::Exp(expr));
                    top_conditions.push(Condition::Or(or_vec.clone()));
                }
                false => top_conditions.push(Condition::Exp(expr)),
            }
            input_tmp = input;
            break;
        }
        input_tmp = input;
    }

    return Ok((input_tmp, top_conditions));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converter::dar_syntax::error::convert_error;
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
        assert_eq!(parse_radix_number("0o37"), Ok(("", NumberLiteral::Hex(31))));
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
            IsActorBase("Skyrim.esm" | 0x00000007) OR
            IsPlayerTeammate() AND
            IsEquippedRightType(3) OR
            IsEquippedRightType(4)
"#;

        let actor = Expression {
            negated: false,
            fn_name: "IsActorBase",
            args: vec![FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x00000007),
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
        match parse_condition(input) {
            Ok(res) => {
                assert_eq!(res, ("", expected));
            }
            Err(err) => match err {
                nom::Err::Incomplete(_) => todo!(),
                nom::Err::Error(err) => {
                    panic!("{}", convert_error(input, err));
                }
                nom::Err::Failure(err) => {
                    panic!("{}", convert_error(input, err));
                }
            },
        };
    }

    #[test]
    fn should_parse_with_space() {
        let input = r#" IsActorBase ( "Skyrim.esm"|0x00000007 ) "#;
        let expected = Condition::And(vec![Condition::Exp(Expression {
            negated: false,
            fn_name: "IsActorBase",
            args: vec![FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x00000007),
            }],
        })]);
        assert_eq!(parse_condition(input), Ok(("", expected)));
    }

    #[test]
    fn should_err_invalid_syntax() {
        let input = "NOT IsActorBase ( \"Skyrim.esm\" | 0x00000007 )OR";
        // assert!(parse_condition(input).is_err());
        match parse_condition(input) {
            Ok(res) => {
                dbg!(res);
                unreachable!()
            }
            Err(err) => match err {
                nom::Err::Incomplete(_) => todo!(),
                nom::Err::Error(err) => {
                    dbg!(&err);
                    println!("{}", convert_error(input, err));
                }
                nom::Err::Failure(err) => {
                    dbg!(&err);
                    println!("{}", convert_error(input, err));
                }
            },
        };
    }

    #[test]
    fn test() {
        let input = r#"
HasMagicEffect("Smooth Animation.esp"|0x000803) AND
NOT IsInCombat() AND
NOT IsAttacking() AND
IsInFaction("Skyrim.esm"|0x000DB1) OR
IsPlayerTeammate() AND
IsEquippedLeftType(1) OR
IsEquippedLeftType(2) OR
IsEquippedLeftType(3) OR
IsEquippedLeftType(4) AND
IsEquippedRightType(3) OR
IsEquippedRightType(4)
     "#;

        println!("{:?}", parse_condition(input));
    }
}
