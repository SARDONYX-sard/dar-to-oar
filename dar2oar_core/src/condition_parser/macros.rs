//! Macros for type conversion of parsed DAR structures into easily serializable OAR structures
use super::ParseError;
use crate::dar_syntax::syntax::FnArg;

/// A trait for safely accessing elements in a vector without causing runtime panics.
///
/// This trait provides methods to access the index of a vector, returning a result
/// with either the desired element or an error if the index is out of bounds.
pub(super) trait GetArg {
    /// The type of the output, which is a result containing a reference to the desired element
    /// or a error if the index is out of bounds.
    ///
    /// Use [Generic Associated Types(GATs)](https://blog.rust-lang.org/2022/10/28/gats-stabilization.html#what-are-gats)
    /// for the `&T` in [`Vec<T>`] because it has the same lifeTime as [Vec].
    type Output<'a>
    where
        Self: 'a;

    /// Access the element at the specified index of the vector.
    ///
    /// # Returns
    ///
    /// A result containing a reference to the desired element or a `Error` if the index is out of bounds.
    fn try_get(&self, index: usize, expected: impl ToString) -> Self::Output<'_>;

    /// Access the element at the specified index of the vector with additional information in case of an error.
    ///
    /// # Returns
    ///
    /// A result containing a reference to the desired element or a `Error` with detailed information if the index is out of bounds.
    fn try_get_real<T>(&self, index: usize, expected: T, actual: T) -> Self::Output<'_>
    where
        T: ToString;
}

impl GetArg for Vec<FnArg<'_>> {
    type Output<'a> = Result<&'a FnArg<'a>, ParseError> where Self: 'a;

    fn try_get(&self, index: usize, expected: impl ToString) -> Self::Output<'_> {
        self.get(index).ok_or(ParseError::UnexpectedValue(
            expected.to_string(),
            format!("None in args[{}]", index),
        ))
    }

    fn try_get_real<T>(&self, index: usize, expected: T, actual: T) -> Self::Output<'_>
    where
        T: ToString,
    {
        self.get(index).ok_or(ParseError::UnexpectedValue(
            expected.to_string(),
            actual.to_string(),
        ))
    }
}

/// [`Vec::get(index)`](https://doc.rust-lang.org/stable/alloc/vec/struct.Vec.html#method.get) & [`TryInto`]
macro_rules! get_try_into {
    ($args:ident[$index:literal], $expected:literal) => {
        <Vec<crate::dar_syntax::syntax::FnArg<'_>> as $crate::condition_parser::macros::GetArg>::try_get(
            &$args, $index, $expected,
        )?
        .try_into()
    };
    ($args:ident[$index:literal], $expected:literal, $actual:literal) => {
        <Vec<crate::dar_syntax::syntax::FnArg<'_>> as $crate::condition_parser::macros::GetArg>::try_get_real(
            &$args, $index, $expected, $actual
        )?
        .try_into()
    };
}
pub(super) use get_try_into;

/// Generate `ConditionSet` &
/// [`Vec::get`](https://doc.rust-lang.org/stable/alloc/vec/struct.Vec.html#method.get)(index) &
/// [`TryInto`] (can use `into` if you need)
macro_rules! gen_cond {
    ($id:ident($field_name:ident, $negated:ident), $args:ident, $expected:literal) => {
        ConditionSet::$id($id {
            negated: $negated,
            $field_name: $crate::condition_parser::macros::get_try_into!($args[0], $expected)?,
            ..Default::default()
        })
    };
    ($id:ident($field_name:ident, $negated:ident), $args:ident, $expected:literal, into) => {
        ConditionSet::$id($id {
          negated: $negated,
          $field_name:
          <Vec<crate::dar_syntax::syntax::FnArg<'_>> as $crate::condition_parser::macros::GetArg>::try_get(&$args, 0, $expected)?.into(),
            ..Default::default()
        })
    };
}
pub(super) use gen_cond;
