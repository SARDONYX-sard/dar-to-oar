//! DAR Plugin value type

use std::borrow::Cow;

use super::{number::form_id, string::string};

use oar_values::PluginValue;
use winnow::{
    Parser,
    combinator::seq,
    error::{
        ModalResult,
        StrContext::{Expected, Label},
        StrContextValue::Description,
    },
};
use winnow_ext::delimited_multispace0;

/// Parse plugin value(e.g. `"Skyrim.esm" | 0x007`)
pub fn parse_plugin_value<'i>(input: &mut &'i str) -> ModalResult<PluginValue<'i>> {
    seq! {
        PluginValue {
            plugin_name: delimited_multispace0(string).map(Cow::Borrowed),
            _: "|",
            form_id: delimited_multispace0(form_id)
        }
    }
    .context(Label("Plugin"))
    .context(Expected(Description(
        r#"Plugin: e.g. `"Skyrim.esm" | 0x007`"#,
    )))
    .parse_next(input)
}
