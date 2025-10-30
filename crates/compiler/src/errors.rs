use parserc::{ControlFlow, Kind, ParseError, Span};

/// Error kinds returns by `compiler`.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum MarkDownError {
    #[error("{0:?}")]
    Kind(Kind),
    #[error("{0:?}: Parsing `horizon` error, {1:?}")]
    Horizon(ControlFlow, Span),
    #[error("{0:?}: Parsing `newline` error, {1:?}")]
    NewLine(ControlFlow, Span),
    #[error("{0:?}: Parsing `leading whitespaces` error, {1:?}")]
    LeadingWhiteSpace(ControlFlow, Span),
    #[error("{0:?}: Parsing `leading #` error, {1:?}")]
    LeadingPounds(ControlFlow, Span),
}

impl From<Kind> for MarkDownError {
    fn from(value: Kind) -> Self {
        match value {
            Kind::Syntax("NewLine", control_flow, span) => {
                MarkDownError::NewLine(control_flow, span)
            }
            _ => MarkDownError::Kind(value),
        }
    }
}

impl ParseError for MarkDownError {
    fn control_flow(&self) -> parserc::ControlFlow {
        match self {
            MarkDownError::Kind(kind) => kind.control_flow(),
            MarkDownError::Horizon(control_flow, _) => *control_flow,
            MarkDownError::NewLine(control_flow, _) => *control_flow,
            MarkDownError::LeadingWhiteSpace(control_flow, _) => *control_flow,
            MarkDownError::LeadingPounds(control_flow, _) => *control_flow,
        }
    }

    fn into_fatal(self) -> Self {
        match self {
            MarkDownError::Kind(kind) => MarkDownError::Kind(kind.into_fatal()),
            MarkDownError::Horizon(_, span) => MarkDownError::Horizon(ControlFlow::Fatal, span),
            MarkDownError::NewLine(_, span) => MarkDownError::NewLine(ControlFlow::Fatal, span),
            MarkDownError::LeadingWhiteSpace(_, span) => {
                MarkDownError::LeadingWhiteSpace(ControlFlow::Fatal, span)
            }
            MarkDownError::LeadingPounds(_, span) => {
                MarkDownError::LeadingPounds(ControlFlow::Fatal, span)
            }
        }
    }

    fn span(&self) -> Span {
        match self {
            MarkDownError::Kind(kind) => kind.span(),
            MarkDownError::Horizon(_, span) => span.clone(),
            MarkDownError::NewLine(_, span) => span.clone(),
            MarkDownError::LeadingWhiteSpace(_, span) => span.clone(),
            MarkDownError::LeadingPounds(_, span) => span.clone(),
        }
    }
}
