use parserc::syntax::Syntax;

use crate::{Horizon, MarkDownInput, NewLine, S};

/// Horizon line block.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Syntax)]
pub struct HorizonLine<I>
where
    I: MarkDownInput,
{
    /// Optional identation
    pub leading_whitespaces: Option<S<I>>,
    /// Main horizon chars: `**..` | `---..` | `___...`
    pub horizon: Horizon<I>,
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

    use crate::{Horizon, HorizonLine, NewLine, S, TokenStream};

    #[test]
    fn test_horizon_line() {
        let mut input = TokenStream::from("     ******   \r\n");

        assert_eq!(
            input.parse(),
            Ok(HorizonLine {
                leading_whitespaces: Some(S(TokenStream::from("     "))),
                horizon: Horizon::Stars(TokenStream::from((5, "******"))),
                trailing_whitespaces: Some(S(TokenStream::from((11, "   ")))),
                new_line: Some(NewLine::CRLR(TokenStream::from((14, "\r\n"))))
            })
        );
    }
}
