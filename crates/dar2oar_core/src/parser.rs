//! Module to convert a parsed DAR into a serializable OAR structure.

use crate::error::{ConvertError, Result};
use dar_syntax::parse_dar_syntax;
use oar_conditions::conditions::Oar;
use std::path::Path;

/// Parse a DAR string and convert it into a vector of [`ConditionSets`] representing an OAR structure.
///
/// This function takes a DAR string as input and parses it into a serializable OAR structure.
/// It returns a [`Result`] containing a vector of [`ConditionSet`] if successful,
/// or a [`ConvertError`] if any parsing or conversion error occurs.
///
/// # Info
/// Now, `path` is only used in case of errors.
pub fn parse_dar2oar<P>(path: P, input: &'_ str) -> Result<Vec<Oar<'_>>, ConvertError>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let dar_ast = parse_dar_syntax(input).map_err(|err| ConvertError::InvalidDarSyntax {
        path: path.to_path_buf(),
        source: err,
    })?;

    #[cfg(feature = "tracing")]
    tracing::debug!(
        "Path: {}
txt -> DAR ast:
{:#?}",
        path.display(),
        dar_ast
    );

    let oar_ast: Oar = dar_ast.into();

    #[cfg(feature = "tracing")]
    tracing::debug!(
        "Path: {}
DAR ast -> OAR ast:
{:#?}",
        path.display(),
        oar_ast
    );

    Ok(oar_ast.into_vec()?)
}
