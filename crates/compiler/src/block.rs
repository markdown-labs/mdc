use parserc::syntax::Syntax;

use crate::{MarkDownInput, NewLine, S, ThematicBreaks};

/// Thematic breaks.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Syntax)]
pub struct ThematicBreaksBlock<I>
where
    I: MarkDownInput,
{
    /// Optional identation
    pub leading_whitespaces: Option<S<I>>,
    /// Main horizon chars: `**..` | `---..` | `___...`
    pub horizon: ThematicBreaks<I>,
    /// Optional trailing whitespace chars.
    pub trailing_whitespaces: Option<S<I>>,
    /// New line broken.
    ///
    /// This section is optional if this line block constitutes the final line of a document.
    pub new_line: Option<NewLine<I>>,
}

#[cfg(test)]
mod tests {
    use parserc::syntax::InputSyntaxExt;

    use crate::{NewLine, S, ThematicBreaks, ThematicBreaksBlock, TokenStream};

    #[test]
    fn test_horizon_line() {
        let mut input = TokenStream::from("     ******   \r\n");

        assert_eq!(
            input.parse(),
            Ok(ThematicBreaksBlock {
                leading_whitespaces: Some(S(TokenStream::from("     "))),
                horizon: ThematicBreaks::Stars(TokenStream::from((5, "******"))),
                trailing_whitespaces: Some(S(TokenStream::from((11, "   ")))),
                new_line: Some(NewLine::CRLR(TokenStream::from((14, "\r\n"))))
            })
        );
    }
}
