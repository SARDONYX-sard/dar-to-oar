//! Parses an actor-related condition based on the provided arguments and condition name.
use super::errors::{ParseError, Result};
use crate::{
    conditions::{CompareValues, ConditionSet, IsActorBase},
    dar_syntax::syntax::FnArg,
    values::{ActorValue, ActorValueType, Cmp, NumericValue},
};

/// Parses an actor-related condition based on the provided arguments and condition name.
/// # Errors
/// If parsing fails.
pub(super) fn parse_actor<'a>(
    condition_name: &'a str,
    mut args: Vec<FnArg<'a>>,
    negated: bool,
) -> Result<ConditionSet<'a>> {
    let mut create_actor_cond = |comparison: Cmp,
                                 actor_value_type: ActorValueType|
     -> Result<ConditionSet<'a>, ParseError> {
        let args_len = args.len();
        if args_len < 2 {
            return Err(ParseError::UnexpectedValue(
                "At least 2 argument is required, but got {arg_len}".into(),
                "".into(),
            ));
        }

        let value_b = args
            .pop()
            .ok_or(ParseError::NotEnoughArguments {
                expected: 2,
                actual: args_len,
            })?
            .try_into()?;
        let actor_value = args
            .pop()
            .ok_or(ParseError::NotEnoughArguments {
                expected: 2,
                actual: args_len,
            })?
            .try_into()?;

        Ok(ConditionSet::CompareValues(CompareValues {
            negated,
            value_a: NumericValue::ActorValue(ActorValue {
                actor_value,
                actor_value_type,
            }),
            comparison,
            value_b: NumericValue::StaticValue(value_b),
            ..Default::default()
        }))
    };

    Ok(match condition_name {
        "IsActorValueEqualTo" => create_actor_cond(Cmp::Eq, ActorValueType::ActorValue)?,
        "IsActorValueLessThan" => create_actor_cond(Cmp::Lt, ActorValueType::ActorValue)?,
        "IsActorValueBaseLessThan" => create_actor_cond(Cmp::Lt, ActorValueType::Base)?,
        "IsActorValueMaxEqualTo" => create_actor_cond(Cmp::Eq, ActorValueType::Max)?,
        "IsActorValueMaxLessThan" => create_actor_cond(Cmp::Lt, ActorValueType::Max)?,
        "IsActorValuePercentageEqualTo" => create_actor_cond(Cmp::Eq, ActorValueType::Percentage)?,
        "IsActorValuePercentageLessThan" => create_actor_cond(Cmp::Lt, ActorValueType::Percentage)?,
        "IsActorBase" => ConditionSet::IsActorBase(IsActorBase {
            actor_base: args.swap_remove(0).try_into()?,
            negated,
            ..Default::default()
        }),
        unknown_condition => {
            return Err(ParseError::UnexpectedValue(
                "IsActor(Value|Base|Max|Percentage)(EqualTo|LessThan)".into(),
                unknown_condition.into(),
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        dar_syntax::syntax::NumberLiteral,
        values::{ActorValue, Cmp, NumericLiteral, NumericValue, PluginValue, StaticValue},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_actor_is_actor_value_equal_to() {
        // test inputs
        let condition_name = "IsActorValueEqualTo";
        let args = vec![
            FnArg::Number(NumberLiteral::Float(3.3)), // actor_value
            FnArg::Number(NumberLiteral::Float(3.5)), // compare value
        ];
        let is_negated = false;

        let result = parse_actor(condition_name, args, is_negated);

        let expected = ConditionSet::CompareValues(CompareValues {
            negated: false,
            value_a: NumericValue::ActorValue(ActorValue {
                actor_value: NumericLiteral::Float(3.3),
                ..Default::default() // NOTE: The DAR contains the actor_type in the function name.
            }),
            comparison: Cmp::Eq,
            value_b: NumericValue::StaticValue(StaticValue { value: 3.5 }),
            ..Default::default()
        });

        match result {
            Ok(result) => {
                assert_eq!(result, expected);
            }
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn test_parse_actor_is_actor_base() {
        let condition_name = "IsActorBase";
        let args = vec![FnArg::PluginValue {
            plugin_name: "Skyrim.esm",
            form_id: NumberLiteral::Hex(0x0000_0007),
        }];
        let is_negated = true;

        let result = parse_actor(condition_name, args, is_negated);

        let expected = Ok(ConditionSet::IsActorBase(IsActorBase {
            negated: true,
            actor_base: PluginValue {
                plugin_name: "Skyrim.esm".into(),
                form_id: "7".into(),
            },
            ..Default::default()
        }));

        assert_eq!(result, expected);
    }
}
