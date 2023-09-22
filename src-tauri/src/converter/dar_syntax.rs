use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while1},
    character::complete::{char, digit1, multispace0, one_of, space0},
    combinator::{map, opt},
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

#[derive(Debug, Clone, PartialEq)]
pub enum NumberLiteral {
    Hex(usize),
    Decimal(isize),
    Float(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression<'a> {
    pub negate: bool,
    pub function_name: &'a str,
    pub arguments: Vec<FnArg<'a>>,
}

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

type IResult<'a, I, O> = nom::IResult<I, O, nom::error::VerboseError<&'a str>>;

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

use nom::error::ParseError;
macro_rules! bail_kind {
    ($input:ident, $kind:ident) => {
        return Err(nom::Err::Error(nom::error::VerboseError::from_error_kind(
            $input,
            nom::error::ErrorKind::$kind,
        )))
    };
}

fn parse_radix_number<'a>(input: &'a str) -> IResult<&str, NumberLiteral> {
    let (input, _) = multispace0(input)?;
    let (input, radix) = alt((tag("0x"), tag("0b"), tag("0o")))(input)?;
    let (input, digits) = digit1(input)?;

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
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
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

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    let (input, _) = multispace0(input)?;
    alt((
        map(tag("AND"), |_| Operator::And),
        map(tag("OR"), |_| Operator::Or),
    ))(input)
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    let (input, _) = multispace0(input)?;
    let (input, negate) = opt(tag("NOT"))(input)?;
    let (input, _) = space0(input)?;
    let (input, (function_name, args)) = parse_fn_call(input)?;

    Ok((
        input,
        Expression {
            negate: negate.is_some(),
            function_name,
            arguments: args,
        },
    ))
}

fn parse_condition<'a>(input: &'a str) -> IResult<&'a str, Condition<'a>> {
    let mut top_conditions = Condition::And(Vec::new());
    let mut or_vec = Vec::new();
    let mut input_tmp = input;
    let mut is_in_or_stmt = false;

    loop {
        let (input, _) = multispace0(input_tmp)?;
        let (input, expr) = parse_expression(input)?;
        let (input, _) = multispace0(input)?;
        let (input, operator) = opt(parse_operator)(input)?;
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_conditions() {
        let input = r#"
            IsActorBase("Skyrim.esm" | 0x00000007) OR
            IsPlayerTeammate() AND
            IsEquippedRightType(3) OR
            IsEquippedRightType(4)
"#;

        let actor = Expression {
            negate: false,
            function_name: "IsActorBase",
            arguments: vec![FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x00000007),
            }],
        };
        let player = Expression {
            negate: false,
            function_name: "IsPlayerTeammate",
            arguments: vec![],
        };
        let equip_r3 = Expression {
            negate: false,
            function_name: "IsEquippedRightType",
            arguments: vec![FnArg::Number(NumberLiteral::Decimal(3))],
        };
        let equip_r4 = Expression {
            negate: false,
            function_name: "IsEquippedRightType",
            arguments: vec![FnArg::Number(NumberLiteral::Decimal(4))],
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
                    println!("{}", nom::error::convert_error(input, err));
                }
                nom::Err::Failure(err) => {
                    println!("{}", nom::error::convert_error(input, err));
                }
            },
        };
    }

    #[test]
    fn should_parse_with_space() {
        let input = r#" IsActorBase ( "Skyrim.esm"|0x00000007 ) "#;
        let expected = Condition::And(vec![Condition::Exp(Expression {
            negate: false,
            function_name: "IsActorBase",
            arguments: vec![FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x00000007),
            }],
        })]);
        assert_eq!(parse_condition(input), Ok(("", expected)));
    }

    #[test]
    fn should_err_invalid_syntax() {
        let input = "NOT IsActorBase ( \"Skyrim.esm\" | 0x00000007 )OR";
        assert!(parse_condition(input).is_err());
    }
}
