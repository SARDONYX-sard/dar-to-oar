//! DAR syntax parser & error handling
mod error;
mod float;
pub mod syntax;

pub use syntax::parse_dar_syntax;
