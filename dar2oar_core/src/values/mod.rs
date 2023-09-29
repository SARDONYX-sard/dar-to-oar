mod actor_value;
mod comparison;
mod direction_value;
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
pub use self::graph_value::{GraphValue, GraphVariableType};
pub use self::keyword_value::{FormValue, Keyword};
pub use self::literal_value::LiteralValue;
pub use self::numeric_literal::NumericLiteral;
pub use self::numeric_value::NumericValue;
pub use self::plugin_value::PluginValue;
pub use self::random_value::RandomValue;
pub use self::static_value::StaticValue;
pub use self::type_value::{TypeValue, WeaponType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum ValueSet {
    ActorValue(ActorValue),
    KeywordValue(LiteralValue),
    NumericValue(StaticValue),
    PluginValue(PluginValue),
    RandomValue(RandomValue),
    TypeValue(TypeValue),
    #[default]
    Unknown,
}
