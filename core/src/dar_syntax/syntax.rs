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
use core::fmt;
use core::mem;
use core::num::ParseIntError;
use winnow::ascii::{
    dec_int, digit1, hex_digit1, multispace0, oct_digit1, take_escaped, till_line_ending, Caseless,
};
use winnow::combinator::{
    alt, delimited, dispatch, eof, fail, opt, preceded, repeat, separated, separated_pair, seq,
};
use winnow::error::{
    AddContext, FromExternalError, PResult, ParserError,
    StrContext::{self, Expected, Label},
    StrContextValue,
};
use winnow::token::{one_of, take, take_while};
use winnow::Parser;

use super::error::ReadableError;
use super::float::float;

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
#[derive(Clone, PartialEq)]
pub enum NumberLiteral {
    /// e.g. 0x007
    Hex(usize),
    /// e.g. 1
    Decimal(isize),
    /// e.g. 1.0
    Float(f32),
}

// Hex debugging display is displayed in hexadecimal notation because it is difficult to understand if it is in decimal.
impl fmt::Debug for NumberLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hex(arg0) => f.debug_tuple("Hex").field(&format!("{arg0:#x}")).finish(),
            Self::Decimal(arg0) => f.debug_tuple("Decimal").field(arg0).finish(),
            Self::Float(arg0) => f.debug_tuple("Float").field(arg0).finish(),
        }
    }
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
    fn push(&mut self, expression: Self) {
        match self {
            Condition::And(inner) | Condition::Or(inner) => inner.push(expression),
            Condition::Exp(_) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Expression cannot push");
            }
        }
    }
}

/// Parsing target.
type Stream<'i> = &'i str;

/// A type alias to allow modification of error types.
type Error<'i> = winnow::error::ContextError;

/// Parse DAR syntax.
pub fn parse_dar_syntax(input: Stream<'_>) -> Result<Condition<'_>, ReadableError> {
    let syntax = input;
    parse_condition::<Error>
        .parse(syntax)
        .map_err(|error| ReadableError::from_parse(error, input))
}

/// Parses a string with surrounding whitespace(0 or more times)
fn delimited_with_multispace0<'i, E>(s: &'static str) -> impl Parser<Stream<'i>, &'i str, E>
where
    E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>,
{
    delimited(multispace0, s, multispace0)
        .context(StrContext::Expected(StrContextValue::StringLiteral(s)))
}

/// single or double quote string
fn string<'i, E>(input: &mut Stream<'i>) -> PResult<&'i str, E>
where
    E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>,
{
    let single_quote = take_escaped(
        take_while(1.., |c| c != '\'' && c != '\\'),
        '\\',
        one_of(['\\', '\'']),
    );

    let double_quote = take_escaped(
        take_while(1.., |c| c != '\"' && c != '\\'),
        '\\',
        one_of(['\\', '"']),
    );

    alt((
        delimited('\"', single_quote, '\"'),
        delimited('"', double_quote, '"'),
    ))
    .context(Label("String"))
    .context(Expected(StrContextValue::Description(
        r#"String: e.g. `"Skyrim.esm"`"#,
    )))
    .parse_next(input)
}

/// Replace a prefixed radix number such as `0x` with Replace with hexadecimal number without prefix.
fn radix_digits<'i, E>(input: &mut Stream<'i>) -> PResult<NumberLiteral, E>
where
    E: ParserError<Stream<'i>>
        + AddContext<Stream<'i>, StrContext>
        + FromExternalError<Stream<'i>, ParseIntError>,
{
    dispatch!(take(2_usize);
        "0b" | "0B" => digit1.try_map(|s| usize::from_str_radix(s, 2))
          .context(StrContext::Label("digit")).context(StrContext::Expected(StrContextValue::Description("binary"))),
        "0o" | "0O" => oct_digit1.try_map(|s| usize::from_str_radix(s, 8))
          .context(StrContext::Label("digit")).context(StrContext::Expected(StrContextValue::Description("octal"))),
        "0d" | "0D" => digit1.try_map(|s: &str| s.parse::<usize>())
          .context(StrContext::Label("digit")).context(StrContext::Expected(StrContextValue::Description("decimal"))),
        "0x" | "0X" => hex_digit1.try_map(|s|usize::from_str_radix(s, 16))
          .context(StrContext::Label("digit")).context(StrContext::Expected(StrContextValue::Description("hexadecimal"))),
        _ => fail.context(StrContext::Label("radix prefix"))
          .context(StrContext::Expected(StrContextValue::StringLiteral("0b")))
          .context(StrContext::Expected(StrContextValue::StringLiteral("0o")))
          .context(StrContext::Expected(StrContextValue::StringLiteral("0d")))
          .context(StrContext::Expected(StrContextValue::StringLiteral("0x"))),
    )
    .map(NumberLiteral::Hex)
    .parse_next(input)
}

/// Parse a number(e.g. "0x123", "123", "12.3")
fn number<'i, E>(input: &mut Stream<'i>) -> PResult<NumberLiteral, E>
where
    E: ParserError<Stream<'i>>
        + AddContext<Stream<'i>, StrContext>
        + FromExternalError<Stream<'i>, ParseIntError>,
{
    alt((
        radix_digits.context(Label("number")),
        float.map(NumberLiteral::Float).context(Label("number")),
        dec_int.map(NumberLiteral::Decimal).context(Label("number")),
        // At this point, if the string `Hi`, etc. is received, the following error report is made.
        fail.context(Label("number"))
            .context(Expected(StrContextValue::Description(
                "radix: e.g. `0x007`",
            )))
            .context(Expected(StrContextValue::Description("float: e.g. `33.0`")))
            .context(Expected(StrContextValue::Description("decimal: e.g. `10`"))),
    ))
    .parse_next(input)
}

/// Parse plugin value(e.g. `"Skyrim.esm" | 0x007`)
fn parse_plugin<'i, E>(input: &mut Stream<'i>) -> PResult<FnArg<'i>, E>
where
    E: ParserError<Stream<'i>>
        + AddContext<Stream<'i>, StrContext>
        + FromExternalError<Stream<'i>, ParseIntError>,
{
    separated_pair(
        delimited(multispace0, string, multispace0),
        "|",
        delimited(multispace0, number, multispace0),
    )
    .map(|(plugin_name, form_id)| FnArg::PluginValue {
        plugin_name,
        form_id,
    })
    .context(Label("Plugin"))
    .context(Expected(StrContextValue::Description(
        r#"Plugin: e.g. `"Skyrim.esm" | 0x007`"#,
    )))
    .parse_next(input)
}

/// Parse function arguments.
fn parse_argument<'i, E>(input: &mut Stream<'i>) -> PResult<FnArg<'i>, E>
where
    E: ParserError<Stream<'i>>
        + AddContext<Stream<'i>, StrContext>
        + FromExternalError<Stream<'i>, ParseIntError>,
{
    alt((
        parse_plugin,
        number.map(FnArg::Number),
        fail.context(Label("function argument"))
            .context(Expected(StrContextValue::Description("plugin")))
            .context(Expected(StrContextValue::StringLiteral("number"))),
    ))
    .parse_next(input)
}

/// Parse identifier
fn ident<'i, E>(input: &mut Stream<'i>) -> PResult<&'i str, E>
where
    E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>,
{
    take_while(1.., |c: char| c.is_alphanumeric() || c == '_')
        .context(Label("Identifier"))
        .context(Expected(StrContextValue::Description(
            "Identifier(e.g. `IsActorBase`)",
        )))
        .parse_next(input)
}

/// Parse function call(with arguments)
///
/// # Expected Syntax Examples
/// ```txt
/// ; Pattern1
/// IsActorBase("Skyrim.esm" | 0x00000007)
///
/// ; Pattern2
/// IsActorValueEqualTo(0x00000007, 30)
/// ```
fn fn_call<'i, E>(input: &mut Stream<'i>) -> PResult<(&'i str, Vec<FnArg<'i>>), E>
where
    E: ParserError<Stream<'i>>
        + AddContext<Stream<'i>, StrContext>
        + FromExternalError<Stream<'i>, ParseIntError>,
{
    seq!(
        ident,
        opt(delimited(
            delimited_with_multispace0("("),
            separated(
                0..,
                delimited(multispace0, parse_argument, multispace0).context(Label("FnArg")),
                ","
            ),
            delimited_with_multispace0(")"),
        ))
        .map(|args| args.unwrap_or_default())
    )
    .context(Label("Function call"))
    .parse_next(input)
}

/// - Expect an AND or OR string.
/// - After AND or OR comes Expression with a line break in between, so the line break is also checked.
fn parse_operator<'i, E>(input: &mut Stream<'i>) -> PResult<Operator, E>
where
    E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>,
{
    preceded(
        multispace0,
        alt((
            Caseless("AND").value(Operator::And),
            Caseless("OR").value(Operator::Or),
        )),
    )
    .context(Label("Operator"))
    .context(Expected(StrContextValue::StringLiteral("AND")))
    .context(Expected(StrContextValue::StringLiteral("OR")))
    .parse_next(input)
}

/// Parse one line DAR Syntax
/// # Expected Syntax examples
/// ```txt
/// NOT IsInCombat()
/// ```
fn parse_expression<'i, E>(input: &mut Stream<'i>) -> PResult<Expression<'i>, E>
where
    E: ParserError<Stream<'i>>
        + AddContext<Stream<'i>, StrContext>
        + FromExternalError<Stream<'i>, ParseIntError>,
{
    seq!(
        _: multispace0,
        opt(Caseless("NOT")).context(Expected(StrContextValue::StringLiteral("NOT")))
            .map(|not| not.is_some()),
        _: multispace0,
        fn_call,
        _: multispace0,
    )
    .map(|(negated, (fn_name, args))| Expression {
        negated,
        fn_name,
        args,
    })
    .parse_next(input)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Parse DAR syntax
fn parse_condition<'i, E>(input: &mut Stream<'i>) -> PResult<Condition<'i>, E>
where
    E: ParserError<Stream<'i>>
        + AddContext<Stream<'i>, StrContext>
        + FromExternalError<Stream<'i>, ParseIntError>,
{
    let mut top_conditions = Condition::And(Vec::new());
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

        let _ = line_comments0(input)?;
        let (expr, operator) = seq!(parse_expression, opt(parse_operator)).parse_next(input)?;
        let _ = line_comments0(input)?;
        let _ = multispace0(input)?;

        #[cfg(feature = "tracing")]
        tracing::trace!("expr: {expr:#?}");
        if let Some(operator) = operator {
            match operator {
                Operator::And => {
                    if is_in_or_stmt {
                        or_vec.push(Condition::Exp(expr));
                        top_conditions.push(Condition::Or(mem::take(&mut or_vec)));
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

            // To support tailing `OR` or `AND` statement.
            if input.is_empty() {
                if is_in_or_stmt {
                    top_conditions.push(Condition::Or(mem::take(&mut or_vec)));
                }
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

            let _ = eof
                .context(Label("End of file"))
                .context(Expected(StrContextValue::Description("end of file")))
                .context(Expected(StrContextValue::Description(
                    "Tailing op: `OR` or `AND`",
                )))
                .context(Expected(StrContextValue::Description(
                    "Conditional statement(if it has op): e.g. `NOT IsInCombat() AND`",
                )))
                .parse_next(input)?;
            break;
        }
    }

    Ok(top_conditions)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Comment starting with ';' until newline
fn line_comment<'i, E>(input: &mut Stream<'i>) -> PResult<Stream<'i>, E>
where
    E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>,
{
    delimited(multispace0, preceded(';', till_line_ending), multispace0)
        .context(Label("Comment"))
        .context(Expected(StrContextValue::Description(
            "Comment(e.g. `; Any String`)",
        )))
        .parse_next(input)
}

/// Comments starting with ';' until newline. 0 or more.
fn line_comments0<'i, E>(input: &mut Stream<'i>) -> PResult<Vec<Stream<'i>>, E>
where
    E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>,
{
    repeat(0.., line_comment).parse_next(input)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! parse_assert {
        ($parser:ident($input:tt), $expected:expr) => {
            match $parser::<Error<'_>>.parse($input) {
                Ok(actual) => assert_eq!(actual, $expected),
                Err(err) => panic!("{}", ReadableError::from_parse(err, $input)),
            }
        };
    }

    #[test]
    fn should_parse_radix_number() {
        parse_assert!(radix_digits("0b1010"), NumberLiteral::Hex(10));
        parse_assert!(radix_digits("0B1010"), NumberLiteral::Hex(10));
        parse_assert!(radix_digits("0o37"), NumberLiteral::Hex(31));
        parse_assert!(radix_digits("0O37"), NumberLiteral::Hex(31));
        parse_assert!(radix_digits("0x00000007"), NumberLiteral::Hex(7));
        parse_assert!(radix_digits("0X1A"), NumberLiteral::Hex(26));
    }

    #[test]
    fn should_error_radix_number() {
        assert!(radix_digits::<Error<'_>>.parse("0z123").is_err());
        assert!(radix_digits::<Error<'_>>.parse("0x").is_err());
    }

    #[test]
    fn should_parse_number() {
        parse_assert!(number("33"), NumberLiteral::Decimal(33));
        parse_assert!(number("33.0"), NumberLiteral::Float(33.0));
        parse_assert!(number("0x00000007"), NumberLiteral::Hex(0x00000007));
    }

    #[test]
    fn should_parse_string() {
        parse_assert!(string(r#""0""#), "0");
        parse_assert!(string(r#""with\"escaped""#), "with\\\"escaped");
    }

    #[test]
    fn should_parse_ident() {
        parse_assert!(ident("IsActorBase"), "IsActorBase");
    }

    #[test]
    fn should_parse_fn_call() {
        let input = r#"IsActorValueLessThan(30, 60)"#;
        let expected = (
            "IsActorValueLessThan",
            vec![
                FnArg::Number(NumberLiteral::Decimal(30)),
                FnArg::Number(NumberLiteral::Decimal(60)),
            ],
        );

        parse_assert!(fn_call(input), expected);
    }

    #[test]
    fn should_parse_comment() {
        let input = r#"
        ; comment
"#;
        parse_assert!(line_comment(input), " comment");
    }

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
            args: vec![FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x00BC_DEF7),
            }],
        };
        let player = Expression {
            negated: true,
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
            args: vec![FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x0000_0007),
            }],
        })]);

        assert_eq!(parse_condition::<Error>.parse(input), Ok(expected));
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

        assert_eq!(parse_condition::<Error>.parse(input), Ok(expected));
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

        assert_eq!(parse_condition::<Error>.parse(input), Ok(expected));
    }
}
