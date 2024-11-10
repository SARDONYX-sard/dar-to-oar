//! DAR function arguments
use super::fn_arg::FnArg;
use crate::dar_syntax::errors::{EmptyCollectionSnafu, Result};
use std::collections::VecDeque;
use winnow::stream::Accumulate;

/// Function args
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FnArgs<'input>(pub VecDeque<FnArg<'input>>);

impl<'input> FnArgs<'input> {
    /// Create a new `FnArgs`
    #[cfg(test)]
    pub const fn new() -> Self {
        Self(VecDeque::new())
    }

    /// Pop the first element and return a `Result`
    pub fn pop_front(&mut self) -> Result<FnArg<'input>> {
        self.0.pop_front().ok_or(EmptyCollectionSnafu.build())
    }
}

// Impl Trait to use `winnow::separated`
//
// NOTE: VecDeque is a std lib, so we can avoid orphan rule errors by impl the wrapped trait.
impl<'a> Accumulate<FnArg<'a>> for FnArgs<'a> {
    #[inline(always)]
    fn initial(capacity: Option<usize>) -> Self {
        capacity.map_or_else(
            || FnArgs(VecDeque::new()),
            |capacity| FnArgs(VecDeque::with_capacity(capacity)),
        )
    }
    #[inline(always)]
    fn accumulate(&mut self, acc: FnArg<'a>) {
        self.0.push_back(acc);
    }
}

/// It is difficult to create test data because `VecDeque` does not have macros like `vec![]`.
///
/// Therefore, we will create a wrapper type macro to make it more convenient for testing.
#[cfg(test)]
macro_rules! fn_args {
    () => (
        $crate::dar_syntax::ast::fn_args::FnArgs::new()
    );
    ( $( $arg:expr ),* $(,)? ) => {
        {
            let mut args = $crate::dar_syntax::ast::fn_args::FnArgs::new();
            $(
                args.0.push_back($arg);
            )*
            args
        }
    };
}
#[cfg(test)]
pub(crate) use fn_args;
