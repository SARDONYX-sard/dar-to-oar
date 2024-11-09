//! Human readable error message
use core::fmt;
use core::ops::Range;
use winnow::error::{ContextError, ParseError};

/// Error struct to represent parsing errors in a more user-friendly way.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadableError {
    /// Title(e.g. Error tittle, Path)
    pub title: String,
    /// Error message
    message: String,
    /// Error position
    span: Range<usize>,
    /// All source lines
    input: String,
}

impl ReadableError {
    /// Constructs [`Self`] from parse error & input.
    pub fn from_parse(error: ParseError<&str, ContextError>, input: &str) -> Self {
        let message = error.inner().to_string();
        let input = input.to_string();
        let start = error.offset();
        let end = (start + 1..)
            .find(|e| input.is_char_boundary(*e))
            .unwrap_or(start);
        Self {
            title: "[DAR Syntax Error]".to_string(),
            message,
            span: start..end,
            input,
        }
    }
}

impl fmt::Display for ReadableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = annotate_snippets::Level::Error.title(&self.title).snippet(
            annotate_snippets::Snippet::source(&self.input)
                .fold(true)
                .annotation(
                    annotate_snippets::Level::Error
                        .span(self.span.clone())
                        .label(&self.message),
                ),
        );
        let renderer = annotate_snippets::Renderer::plain();
        let rendered = renderer.render(message);
        rendered.fmt(f)
    }
}

impl std::error::Error for ReadableError {}
