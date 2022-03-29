use crate::{options::RegexFlavor, span::Span};

use super::{Diagnostic, ParseError, ParseErrorKind};

/// An error that can occur during parsing or compiling
#[derive(Debug, Clone, thiserror::Error)]
pub struct CompileError {
    pub(super) kind: CompileErrorKind,
    pub(super) span: Option<Span>,
}

impl CompileError {
    /// Create a [Diagnostic] from this error.
    pub fn diagnostic(self, source_code: &str) -> Diagnostic {
        Diagnostic::from_compile_error(self, source_code)
    }
}

impl core::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(span) = self.span {
            write!(f, "{}\n  at {}", self.kind, span)
        } else {
            self.kind.fmt(f)
        }
    }
}

impl From<ParseError> for CompileError {
    fn from(e: ParseError) -> Self {
        CompileError { kind: CompileErrorKind::ParseError(e.kind), span: e.span }
    }
}

/// An error kind (without span) that can occur during parsing or compiling
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum CompileErrorKind {
    #[error("Parse error: {}", .0)]
    ParseError(ParseErrorKind),

    #[error("Compile error: Unsupported feature `{}` in the `{:?}` regex flavor", .0.name(), .1)]
    Unsupported(Feature, RegexFlavor),

    #[error("Group references this large aren't supported")]
    HugeReference,

    #[error("Reference to unknown group. There is no group number {}", .0)]
    UnknownReferenceNumber(i32),

    #[error("Reference to unknown group. There is no group named `{}`", .0)]
    UnknownReferenceName(String),

    #[error("Compile error: Group name `{}` used multiple times", .0)]
    NameUsedMultipleTimes(String),

    #[error("Compile error: This character class is empty")]
    EmptyClass,

    #[error("Compile error: This negated character class matches nothing")]
    EmptyClassNegated,

    #[error("Capturing groups within `let` statements are currently not supported")]
    CaptureInLet,

    #[error("References within `let` statements are currently not supported")]
    ReferenceInLet,

    #[error("Variable doesn't exist")]
    UnknownVariable,

    #[error("Variables can't be used recursively")]
    RecursiveVariable,

    #[error("Compile error: {}", .0)]
    Other(&'static str),
}

impl CompileErrorKind {
    pub(crate) fn at(self, span: Span) -> CompileError {
        CompileError { kind: self, span: Some(span) }
    }
}

/// Regex feature, possibly unsupported
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Feature {
    NamedCaptureGroups,
    Lookaround,
    Grapheme,
    UnicodeBlock,
    UnicodeProp,
    Backreference,
    ForwardReference,
    RelativeReference,
    NonNegativeRelativeReference,
    NegativeShorthandW,
}

impl Feature {
    fn name(self) -> &'static str {
        match self {
            Feature::NamedCaptureGroups => "named capture groups",
            Feature::Lookaround => "lookahead/behind",
            Feature::Grapheme => "grapheme cluster matcher (\\X)",
            Feature::UnicodeBlock => "Unicode blocks (\\p{InBlock})",
            Feature::UnicodeProp => "Unicode properties (\\p{Property})",
            Feature::Backreference => "Backreference",
            Feature::ForwardReference => "Forward reference",
            Feature::RelativeReference => "Relative backreference",
            Feature::NonNegativeRelativeReference => "Non-negative relative backreference",
            Feature::NegativeShorthandW => "Negative `\\w` shorthand in character class",
        }
    }
}
