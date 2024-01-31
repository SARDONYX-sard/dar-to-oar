//! DAR Condition values
mod actor_value;
mod comparison;
mod direction_value;
mod form_value;
mod graph_value;
mod keyword_value;
mod literal_value;
mod numeric_literal;
mod numeric_value;
mod plugin_value;
mod random_value;
mod static_value;
mod type_value;

pub use self::actor_value::{ActorValue, ActorValueType};
pub use self::comparison::Cmp;
pub use self::direction_value::{Direction, DirectionValue};
pub use self::form_value::FormValue;
#[allow(unused)]
pub use self::graph_value::{GraphValue, GraphVariableType};
pub use self::keyword_value::Keyword;
pub use self::literal_value::LiteralValue;
pub use self::numeric_literal::NumericLiteral;
pub use self::numeric_value::NumericValue;
pub use self::plugin_value::PluginValue;
pub use self::random_value::RandomValue;
pub use self::static_value::StaticValue;
#[allow(unused)]
pub use self::type_value::{TypeValue, WeaponType};
use serde::{Deserialize, Serialize};

/// DAR variable set
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum ValueSet {
    /// Person and its internal value
    ActorValue(ActorValue),
    /// Keyword ID
    KeywordValue(LiteralValue),
    /// Just f32 value
    NumericValue(StaticValue),
    /// Pair plugin name & ID
    PluginValue(PluginValue),
    /// A value with a range, used for randomization.
    RandomValue(RandomValue),
    /// Weapon type
    TypeValue(TypeValue),
    /// Unknown value
    #[default]
    Unknown,
}
