// SPDX-License-Identifier: MIT
//! `HexDump` Display(For binary)/XML human-readable error message
//! This code is a fork of winnow's docs.
//!
//! # Ref
//! - [MIT License](https://github.com/winnow-rs/winnow/blob/v0.7.10/LICENSE-MIT)
//! - [Code](https://github.com/winnow-rs/winnow/blob/v0.7.10/src/error.rs#L1316)
use winnow::error::{ContextError, ErrMode, ParseError, StrContext};

/// Error struct to represent parsing errors in a more user-friendly way.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ReadableError {
    /// Title(e.g. Error tittle, Path)
    title: String,
    /// Error message
    message: String,
    /// Error position
    span: core::ops::Range<usize>,
    /// All source lines
    input: String,
}

impl ReadableError {
    /// Constructs [`Self`] from parse error & input.
    #[inline]
    pub fn from_parse(error: ParseError<&str, ContextError>) -> Self {
        let message = error.inner().to_string();
        let input = (*error.input()).to_string();
        let span = error.char_span();
        Self {
            title: "Parse error".to_string(),
            message,
            span,
            input,
        }
    }

    /// Constructs [`Self`] from parse error & input.
    #[inline]
    pub fn from_context<T>(error: ErrMode<ContextError>, input: T, err_pos: usize) -> Self
    where
        T: core::fmt::Display,
    {
        let (labels, message) = error
            .map(|ctx_err| {
                ctx_err.cause().map_or_else(
                    || {
                        let mut labels = String::new();
                        let mut msg = "expected ".to_string();

                        for ctx in ctx_err.context() {
                            match ctx {
                                StrContext::Label(label) => {
                                    labels += " <- ";
                                    labels += label;
                                }
                                StrContext::Expected(expected) => {
                                    msg += &expected.to_string();
                                }
                                _ => (),
                            }
                        }
                        (labels, msg)
                    },
                    |cause| (String::new(), cause.to_string()),
                )
            })
            .into_inner()
            .unwrap_or_default();

        let input = input.to_string();
        let span = char_boundary(input.as_bytes(), err_pos);

        Self {
            title: labels,
            message,
            span,
            input,
        }
    }
}

impl core::fmt::Display for ReadableError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // https://docs.rs/annotate-snippets/0.12.0/annotate_snippets/index.html
        use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

        let message = &[
            Group::with_title(Level::ERROR.primary_title(self.title.clone())).element(
                Snippet::source(&self.input)
                    .line_start(self.span.start)
                    .fold(true)
                    .annotation(
                        AnnotationKind::Primary
                            .span(self.span.clone())
                            .label(&self.message),
                    ),
            ),
        ];
        let renderer = Renderer::plain();
        let rendered = renderer.render(message);
        rendered.fmt(f)
    }
}

impl std::error::Error for ReadableError {}

/// winnow method
fn char_boundary(input: &[u8], offset: usize) -> core::ops::Range<usize> {
    let len = input.len();
    if offset == len {
        return offset..offset;
    }

    /// Taken from `core::num`
    const fn is_utf8_char_boundary(b: u8) -> bool {
        // This is bit magic equivalent to: b < 128 || b >= 192
        (b as i8) >= -0x40
    }

    let start = (0..(offset + 1).min(len))
        .rev()
        .find(|i| input.get(*i).copied().map_or(false, is_utf8_char_boundary))
        .unwrap_or(0);
    let end = (offset + 1..len)
        .find(|i| input.get(*i).copied().map_or(false, is_utf8_char_boundary))
        .unwrap_or(len);
    start..end
}
