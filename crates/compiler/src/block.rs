use parserc::syntax::Syntax;

use crate::{Horizon, MarkDownInput, NewLine, S};

/// Horizon line block.
#[derive(Debug, Clone, Syntax)]
pub struct HorizonLine<I>
where
    I: MarkDownInput,
{
    /// Optional identation
    pub leading_whitespaces: Option<S<I>>,
    /// Main horizon chars: `**..` | `---..` | `___...`
    pub horizon: Horizon<I>,
    /// Optional tailing whitespace chars.
    pub tailing_whitespaces: Option<S<I>>,
    /// New line broken.
    ///
    /// This section is optional if this line block constitutes the final line of a document.
    pub new_line: Option<NewLine<I>>,
}
