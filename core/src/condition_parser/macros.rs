//! Macros for type conversion of parsed DAR structures into easily serializable OAR structures
use super::ParseError;
use crate::dar_syntax::syntax::FnArg;

/// Attempts to remove and return the element at the specified index from the given vector.
///
/// # Errors
/// If the index is out of bounds, it returns a `ParseError::NotEnoughArguments` error.
pub fn try_swap_remove<'a>(
    args: &mut Vec<FnArg<'a>>,
    index: usize,
) -> Result<FnArg<'a>, ParseError> {
    let args_len = args.len();
    if args_len < index {
        return Err(ParseError::NotEnoughArguments {
            expected: index,
            actual: args_len,
        });
    }

    Ok(args.swap_remove(index))
}

/// Generate `ConditionSet` &
/// [`Vec::get`](https://doc.rust-lang.org/stable/alloc/vec/struct.Vec.html#method.get)(index) &
/// [`TryInto`] (can use `into` if you need)
macro_rules! gen_cond {
    ($id:ident($field_name:ident, $negated:ident), $args:ident, $expected:literal) => {
        ConditionSet::$id($id {
            negated: $negated,
            $field_name: $crate::condition_parser::macros::try_swap_remove(&mut $args, 0)?
                .try_into()?,
            ..Default::default()
        })
    };
    ($id:ident($field_name:ident, $negated:ident), $args:ident, $expected:literal, into) => {
        ConditionSet::$id($id {
            negated: $negated,
            $field_name: $crate::condition_parser::macros::try_swap_remove(&mut $args, 0)?.into(),
            ..Default::default()
        })
    };
}
pub(super) use gen_cond;
