use parserc::{ControlFlow, ParseError, Span};

/// Error kind for markdown document parsing.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Kind {
    #[error("line ending chars")]
    LineEnding,
    #[error("leading whitespaces")]
    LeadingWhiteSpace,
    #[error("leading #..")]
    LeadingPounds,
    #[error("escaped")]
    Escaped,
    #[error("html5 entities")]
    Entity,
    #[error("thematic breaks")]
    Thematic,
    #[error("whitespaces")]
    S,
    #[error("non-empty whitespace chars")]
    S1,
    #[error("ATX heading")]
    ATXHeading,
    #[error("blank line")]
    BlankLine,
    #[error("identation non-blank chunk")]
    IdentationNonblankChunk,
    #[error("identation blank chunk")]
    IdentationBlankChunk,
}

impl Kind {
    /// Map error to this kind.
    pub fn map(self) -> impl FnOnce(MarkDownError) -> MarkDownError {
        |err: MarkDownError| MarkDownError::Kind(self, err.control_flow(), err.span())
    }
}

/// Error kinds returns by `compiler`.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum MarkDownError {
    #[error("{0:?}")]
    Other(parserc::Kind),
    #[error("{1:?}: Parsing `{0:?}` error, {1:?}")]
    Kind(Kind, ControlFlow, Span),
}

impl From<parserc::Kind> for MarkDownError {
    fn from(value: parserc::Kind) -> Self {
        match value {
            parserc::Kind::Syntax("NewLine", control_flow, span) => {
                MarkDownError::Kind(Kind::LineEnding, control_flow, span)
            }
            _ => MarkDownError::Other(value),
        }
    }
}

impl ParseError for MarkDownError {
    fn control_flow(&self) -> parserc::ControlFlow {
        match self {
            MarkDownError::Other(kind) => kind.control_flow(),
            MarkDownError::Kind(_, control_flow, _) => *control_flow,
        }
    }

    fn into_fatal(self) -> Self {
        match self {
            MarkDownError::Other(kind) => MarkDownError::Other(kind.into_fatal()),
            MarkDownError::Kind(kind, _, span) => {
                MarkDownError::Kind(kind, ControlFlow::Fatal, span)
            }
        }
    }

    fn span(&self) -> Span {
        match self {
            MarkDownError::Other(kind) => kind.span(),
            MarkDownError::Kind(_, _, span) => span.clone(),
        }
    }
}
