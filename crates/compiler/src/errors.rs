use parserc::{ControlFlow, Kind, ParseError, Span};

/// Error kinds returns by `compiler`.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum MarkDownError {
    #[error("{0:?}")]
    Kind(Kind),
    #[error("{0:?}: Parsing `newline` error, {1:?}")]
    LineEnding(ControlFlow, Span),
    #[error("{0:?}: Parsing `leading whitespaces` error, {1:?}")]
    LeadingWhiteSpace(ControlFlow, Span),
    #[error("{0:?}: Parsing `leading #` error, {1:?}")]
    LeadingPounds(ControlFlow, Span),
    #[error("{0:?}: Parsing `escaped` error, {1:?}")]
    Escaped(ControlFlow, Span),
    #[error("{0:?}: Parsing `html5 entity` error, {1:?}")]
    Entity(ControlFlow, Span),
    #[error("{0:?}: Parsing `thematic breaks` error, {1:?}")]
    Thematic(ControlFlow, Span),
    #[error("{0:?}: Parsing `Whitespace` error, {1:?}")]
    S(ControlFlow, Span),
}

impl From<Kind> for MarkDownError {
    fn from(value: Kind) -> Self {
        match value {
            Kind::Syntax("NewLine", control_flow, span) => {
                MarkDownError::LineEnding(control_flow, span)
            }
            _ => MarkDownError::Kind(value),
        }
    }
}

impl ParseError for MarkDownError {
    fn control_flow(&self) -> parserc::ControlFlow {
        match self {
            MarkDownError::Kind(kind) => kind.control_flow(),
            MarkDownError::LineEnding(control_flow, _) => *control_flow,
            MarkDownError::LeadingWhiteSpace(control_flow, _) => *control_flow,
            MarkDownError::LeadingPounds(control_flow, _) => *control_flow,
            MarkDownError::Escaped(control_flow, _) => *control_flow,
            MarkDownError::Entity(control_flow, _) => *control_flow,
            MarkDownError::Thematic(control_flow, _) => *control_flow,
            MarkDownError::S(control_flow, _) => *control_flow,
        }
    }

    fn into_fatal(self) -> Self {
        match self {
            MarkDownError::Kind(kind) => MarkDownError::Kind(kind.into_fatal()),
            MarkDownError::S(_, span) => MarkDownError::S(ControlFlow::Fatal, span),
            MarkDownError::LineEnding(_, span) => {
                MarkDownError::LineEnding(ControlFlow::Fatal, span)
            }
            MarkDownError::LeadingWhiteSpace(_, span) => {
                MarkDownError::LeadingWhiteSpace(ControlFlow::Fatal, span)
            }
            MarkDownError::LeadingPounds(_, span) => {
                MarkDownError::LeadingPounds(ControlFlow::Fatal, span)
            }
            MarkDownError::Escaped(_, span) => MarkDownError::Escaped(ControlFlow::Fatal, span),
            MarkDownError::Entity(_, span) => MarkDownError::Entity(ControlFlow::Fatal, span),
            MarkDownError::Thematic(_, span) => MarkDownError::Entity(ControlFlow::Fatal, span),
        }
    }

    fn span(&self) -> Span {
        match self {
            MarkDownError::Kind(kind) => kind.span(),
            MarkDownError::S(_, span) => span.clone(),
            MarkDownError::LineEnding(_, span) => span.clone(),
            MarkDownError::LeadingWhiteSpace(_, span) => span.clone(),
            MarkDownError::LeadingPounds(_, span) => span.clone(),
            MarkDownError::Escaped(_, span) => span.clone(),
            MarkDownError::Entity(_, span) => span.clone(),
            MarkDownError::Thematic(_, span) => span.clone(),
        }
    }
}
