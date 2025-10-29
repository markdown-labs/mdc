use parserc::{ControlFlow, Kind, ParseError, Span};

/// Error kinds returns by `compiler`.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum MarkDownError {
    #[error("{0:?}: {1}")]
    Kind(ControlFlow, Kind),
    #[error("{0:?}: Parsing `horizon` error, {1:?}")]
    Horizon(ControlFlow, Span),
}

impl From<(ControlFlow, Kind)> for MarkDownError {
    fn from(value: (ControlFlow, Kind)) -> Self {
        MarkDownError::Kind(value.0, value.1)
    }
}

impl ParseError for MarkDownError {
    fn control_flow(&self) -> parserc::ControlFlow {
        match self {
            MarkDownError::Kind(control_flow, _) => *control_flow,
            MarkDownError::Horizon(control_flow, _) => *control_flow,
        }
    }

    fn into_fatal(self) -> Self {
        match self {
            MarkDownError::Kind(_, kind) => MarkDownError::Kind(ControlFlow::Fatal, kind),
            MarkDownError::Horizon(_, span) => MarkDownError::Horizon(ControlFlow::Fatal, span),
        }
    }
}
