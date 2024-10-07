//! Wrapper for [`WeaponType`]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_untagged::UntaggedEnumVisitor;

use super::NumericLiteral;

/// Wrapper for [`WeaponType`]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeValue {
    /// Weapon type value
    pub value: WeaponType,
}

/// Weapon type enumeration
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum WeaponType {
    /// -1.0
    Other = -1,
    #[default]
    /// 0.0
    Unarmed = 0,
    /// 1.0
    Sword,
    /// 2.0
    Dagger,
    /// 3.0
    WarAxe,
    /// 4.0
    Mace,
    /// 5.0
    Greatsword,
    /// 6.0
    Battleaxe,
    /// 7.0
    Bow,
    /// 8.0
    Staff,
    /// 9.0
    Crossbow,
    /// 10.0
    Warhammer,
    /// 11.0
    Shield,
    /// 12.0
    AlterationSpell,
    /// 13.0
    IllusionSpell,
    /// 14.0
    DestructionSpell,
    /// 15.0
    ConjurationSpell,
    /// 16.0
    RestorationSpell,
    /// 17.0
    Scroll,
    /// 18.0
    Torch,
}

impl TryFrom<i64> for WeaponType {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            -1 => Self::Other,
            0 => Self::Unarmed,
            1 => Self::Sword,
            2 => Self::Dagger,
            3 => Self::WarAxe,
            4 => Self::Mace,
            5 => Self::Greatsword,
            6 => Self::Battleaxe,
            7 => Self::Bow,
            8 => Self::Staff,
            9 => Self::Crossbow,
            10 => Self::Warhammer,
            11 => Self::Shield,
            12 => Self::AlterationSpell,
            13 => Self::IllusionSpell,
            14 => Self::DestructionSpell,
            15 => Self::ConjurationSpell,
            16 => Self::RestorationSpell,
            17 => Self::Scroll,
            18 => Self::Torch,
            _ => return Err("Invalid value for WeaponType"),
        })
    }
}

impl TryFrom<NumericLiteral> for WeaponType {
    type Error = &'static str;

    fn try_from(value: NumericLiteral) -> Result<Self, Self::Error> {
        match value {
            NumericLiteral::Hex(num) => match num {
                1..=18 => Ok((num as i64).try_into()?),
                _ => Err("Got hex, Out of range 1..=18"),
            },
            NumericLiteral::Decimal(num) => match num {
                -1..=18 => Ok((num as i64).try_into()?),
                _ => Err("Got Decimal, Out of range -1..=18"),
            },
            NumericLiteral::Float(num) => Ok(num.try_into()?),
        }
    }
}

impl TryFrom<f32> for WeaponType {
    type Error = &'static str;

    fn try_from(float: f32) -> Result<Self, Self::Error> {
        Ok(match float {
            x if (-1.0..0.0).contains(&x) => Self::Other,
            x if (0.0..1.0).contains(&x) => Self::Unarmed,
            x if (1.0..2.0).contains(&x) => Self::Sword,
            x if (2.0..3.0).contains(&x) => Self::Dagger,
            x if (3.0..4.0).contains(&x) => Self::WarAxe,
            x if (4.0..5.0).contains(&x) => Self::Mace,
            x if (5.0..6.0).contains(&x) => Self::Greatsword,
            x if (6.0..7.0).contains(&x) => Self::Battleaxe,
            x if (7.0..8.0).contains(&x) => Self::Bow,
            x if (8.0..9.0).contains(&x) => Self::Staff,
            x if (9.0..10.0).contains(&x) => Self::Crossbow,
            x if (10.0..11.0).contains(&x) => Self::Warhammer,
            x if (11.0..12.0).contains(&x) => Self::Shield,
            x if (12.0..13.0).contains(&x) => Self::AlterationSpell,
            x if (13.0..14.0).contains(&x) => Self::IllusionSpell,
            x if (14.0..15.0).contains(&x) => Self::DestructionSpell,
            x if (15.0..16.0).contains(&x) => Self::ConjurationSpell,
            x if (16.0..17.0).contains(&x) => Self::RestorationSpell,
            x if (17.0..18.0).contains(&x) => Self::Scroll,
            x if (18.0..19.0).contains(&x) => Self::Torch,
            _ => return Err("Expected -1.0..=18.0"),
        })
    }
}

impl From<WeaponType> for f64 {
    fn from(value: WeaponType) -> Self {
        match value {
            WeaponType::Other => -1.0,
            WeaponType::Unarmed => 0.0,
            WeaponType::Sword => 1.0,
            WeaponType::Dagger => 2.0,
            WeaponType::WarAxe => 3.0,
            WeaponType::Mace => 4.0,
            WeaponType::Greatsword => 5.0,
            WeaponType::Battleaxe => 6.0,
            WeaponType::Bow => 7.0,
            WeaponType::Staff => 8.0,
            WeaponType::Crossbow => 9.0,
            WeaponType::Warhammer => 10.0,
            WeaponType::Shield => 11.0,
            WeaponType::AlterationSpell => 12.0,
            WeaponType::IllusionSpell => 13.0,
            WeaponType::DestructionSpell => 14.0,
            WeaponType::ConjurationSpell => 15.0,
            WeaponType::RestorationSpell => 16.0,
            WeaponType::Scroll => 17.0,
            WeaponType::Torch => 18.0,
        }
    }
}

impl Serialize for WeaponType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the variant as a floating-point number.
        let value: f64 = self.clone().into();
        value.serialize(serializer)
    }
}

// NOTE: Numeric comparison with float(f32) does not work correctly, so cast from f64 to i64 and deserialize
// See: https://github.com/rust-lang/rust/issues/41620
impl<'de> Deserialize<'de> for WeaponType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        UntaggedEnumVisitor::new()
            .f32(|float| {
                float.try_into().map_err(|_err| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Float(float.into()),
                        &r#"-1.0..=18.0"#,
                    )
                })
            })
            .f64(|float| {
                (float as f32).try_into().map_err(|_err| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Float(float),
                        &r#"-1.0..=18.0"#,
                    )
                })
            })
            .deserialize(deserializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_type_value() -> Result<()> {
        let type_value = TypeValue {
            value: WeaponType::Other,
        };

        let expected = r#"{
  "value": -1.0
}"#;
        let serialized = serde_json::to_string_pretty(&type_value)?;
        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_type_value() -> Result<()> {
        let json_str = r#"{
            "value": 18.0
        }"#;

        let deserialized: TypeValue = serde_json::from_str(json_str)?;
        let expected = TypeValue {
            value: WeaponType::Torch,
        };

        assert_eq!(deserialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_type_value2() -> Result<()> {
        let json_str = r#"{
            "value": 5.0
        }"#;

        let deserialized: TypeValue = serde_json::from_str(json_str)?;
        let expected = TypeValue {
            value: WeaponType::Greatsword,
        };

        assert_eq!(deserialized, expected);
        Ok(())
    }
}
