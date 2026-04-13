//! Wrapper for [`WeaponType`]
use crate::ValueError;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_untagged::UntaggedEnumVisitor;

/// Wrapper for [`WeaponType`]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeValue {
    /// Weapon type value
    pub value: WeaponType,
}

impl From<WeaponType> for TypeValue {
    #[inline]
    fn from(value: WeaponType) -> Self {
        Self { value }
    }
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

impl TryFrom<f32> for WeaponType {
    type Error = ValueError;

    fn try_from(float: f32) -> Result<Self, Self::Error> {
        Ok(match float {
            -1.0..0.0 => Self::Other,
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
            invalid => {
                return Err(ValueError::CastError {
                    expected: "-1..18".into(),
                    actual: invalid.to_string(),
                });
            }
        })
    }
}

impl Serialize for WeaponType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the variant as a floating-point number.
        serializer.serialize_str(match self {
            Self::Other => "-1.0",
            Self::Unarmed => "0.0",
            Self::Sword => "1.0",
            Self::Dagger => "2.0",
            Self::WarAxe => "3.0",
            Self::Mace => "4.0",
            Self::Greatsword => "5.0",
            Self::Battleaxe => "6.0",
            Self::Bow => "7.0",
            Self::Staff => "8.0",
            Self::Crossbow => "9.0",
            Self::Warhammer => "10.0",
            Self::Shield => "11.0",
            Self::AlterationSpell => "12.0",
            Self::IllusionSpell => "13.0",
            Self::DestructionSpell => "14.0",
            Self::ConjurationSpell => "15.0",
            Self::RestorationSpell => "16.0",
            Self::Scroll => "17.0",
            Self::Torch => "18.0",
        })
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
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_type_value() -> Result<(), serde_json::Error> {
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
    fn should_deserialize_type_value() -> Result<(), serde_json::Error> {
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
    fn should_deserialize_type_value2() -> Result<(), serde_json::Error> {
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
