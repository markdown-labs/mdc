use parserc::syntax::Syntax;

use crate::{LeadingPounds, LeadingWhiteSpace, MarkDownInput, NewLine, S, ThematicBreaks};

/// Thematic breaks.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThematicBreaksLine<I>
where
    I: MarkDownInput,
{
    /// Optional identation
    pub leading_whitespaces: LeadingWhiteSpace<I, 3>,
    /// Main horizon chars: `**..` | `---..` | `___...`
    pub horizon: ThematicBreaks<I>,
    /// Optional trailing whitespace chars.
    pub trailing_whitespaces: S<I>,
    /// New line broken.
    ///
    /// This section is optional if this line block constitutes the final line of a document.
    pub new_line: Option<NewLine<I>>,
}

/// ATX heading block.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HeadingLine<I>
where
    I: MarkDownInput,
{
    /// leading whitespace chars which maximum length is limits to 5
    pub leading_whitespaces: LeadingWhiteSpace<I, 5>,
    /// leading unescaped `#` chars.
    pub leading_pounds: LeadingPounds<I>,
    /// trailing whitespaces.
    pub trailing_whitespaces: S<I>,
    /// New line broken.
    ///
    /// This section is optional if this line block constitutes the final line of a document.
    pub new_line: Option<NewLine<I>>,
}

#[cfg(test)]
mod tests {
    use parserc::{ControlFlow, Span, syntax::InputSyntaxExt};

    use crate::{
        LeadingWhiteSpace, MarkDownError, NewLine, S, ThematicBreaks, ThematicBreaksLine,
        TokenStream,
    };

    #[test]
    fn test_thematic_breaks_line() {
        let mut input = TokenStream::from("   ******   \r\n");

        assert_eq!(
            input.parse(),
            Ok(ThematicBreaksLine {
                leading_whitespaces: LeadingWhiteSpace(TokenStream::from("   ")),
                horizon: ThematicBreaks::Stars(TokenStream::from((3, "******"))),
                trailing_whitespaces: S(TokenStream::from((9, "   "))),
                new_line: Some(NewLine::CRLR(TokenStream::from((12, "\r\n"))))
            })
        );

        let mut input = TokenStream::from("    ******   \r\n");

        assert_eq!(
            input.parse::<ThematicBreaksLine<_>>(),
            Err(MarkDownError::LeadingWhiteSpace(
                ControlFlow::Recovable,
                Span::Range(0..15),
            ))
        );
    }
}
