#[macro_export]
macro_rules! get_arg {
    ($args:ident[$index:literal], $expected:literal) => {
        $args.get($index).ok_or(ParseError::UnexpectedValue(
            $expected.to_string(),
            format!("None in args[{}]", $index),
        ))
    };
    ($args:ident[$index:literal], $expected:literal, $actual:literal) => {
        $args.get($index).ok_or(ParseError::UnexpectedValue(
            $expected.to_string(),
            $actual.to_string(),
        ))
    };
}

/// get arg & into
///
/// Return early if \[$index\]th of args cannot be obtained. After that, do into.
#[macro_export]
macro_rules! get_into {
    ($args:ident[$index:literal], $expected:literal) => {
        $crate::get_arg!($args[$index], $expected)?.into()
    };
    ($args:ident[$index:literal], $expected:literal, $actual:literal) => {
        $crate::get_arg!($args[$index], $expected, $actual)?.into()
    };
}

/// get arg & try_into
#[macro_export]
macro_rules! get_try_into {
    ($args:ident[$index:literal], $expected:literal) => {
        $crate::get_arg!($args[$index], $expected)?.try_into()
    };
    ($args:ident[$index:literal], $expected:literal, $actual:literal) => {
        $crate::get_arg!($args[$index], $expected, $actual)?.try_into()
    };
}

// Generate COnditionSet & get arg then try_into(exec `into()` if use into Option)
/// $id:ident, $field_name:ident, $args:ident $negated:expr, $expected:literal, into(This is Option use into)
#[macro_export]
macro_rules! gen_cond {
    ($id:ident($field_name:ident, $negated:ident), $args:ident, $expected:literal) => {
        ConditionSet::$id($id {
            negated: $negated,
            $field_name: $crate::get_try_into!($args[0], $expected)?,
            ..Default::default()
        })
    };
    ($id:ident($field_name:ident, $negated:ident), $args:ident, $expected:literal, into) => {
        ConditionSet::$id($id {
            negated: $negated,
            $field_name: $crate::get_into!($args[0], $expected),
            ..Default::default()
        })
    };
}
