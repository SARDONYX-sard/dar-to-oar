//! Macros for type conversion of parsed DAR structures into easily serializable OAR structures

/// Generate `ConditionSet` &
/// [`Vec::get`](https://doc.rust-lang.org/stable/alloc/vec/struct.Vec.html#method.get)(index) &
/// [`TryInto`] (can use `into` if you need)
macro_rules! gen_cond {
    ($id:ident($field_name:ident, $negated:ident), $args:ident, $expected:literal) => {
        ConditionSet::$id($id {
            negated: $negated,
            $field_name: $args.pop_front()?.try_into()?,
            ..Default::default()
        })
    };
    ($id:ident($field_name:ident, $negated:ident), $args:ident, $expected:literal, into) => {
        ConditionSet::$id($id {
            negated: $negated,
            $field_name: $args.pop_front()?.into(),
            ..Default::default()
        })
    };
}
pub(super) use gen_cond;
