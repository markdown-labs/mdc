use crate::{IndentationFrom, MarkDownInput};

/// Non-blank lines, each preceded by four or more spaces of indentation.
///
/// See [`https://spec.commonmark.org/0.31.2/#fenced-code-blocks`]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndentedChunk<I>
where
    I: MarkDownInput,
{
    /// Preceded by four or more spaces of indentation.
    pub identation: IndentationFrom<I, 4>,
}
