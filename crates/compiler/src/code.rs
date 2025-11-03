use std::cmp;

use parserc::{
    ControlFlow, Parser, Span,
    syntax::{InputSyntaxExt, Syntax},
    take_till,
};

use crate::{IndentationFrom, Kind, LineEnding, MarkDownError, MarkDownInput, S};

/// Non-blank lines, each preceded by four or more spaces of indentation.
///
/// See [`https://spec.commonmark.org/0.31.2/#fenced-code-blocks`]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndentedNonblankLine<I>
where
    I: MarkDownInput,
{
    /// Preceded by four or more spaces of indentation.
    pub identation: IndentationFrom<I, 4>,
    /// Non-blank chars
    pub content: I,
    /// Optional line ending chars
    pub line_ending: Option<LineEnding<I>>,
}

impl<I> Syntax<I> for IndentedNonblankLine<I>
where
    I: MarkDownInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let identation: IndentationFrom<_, _> =
            input.parse().map_err(Kind::IdentationNonblankChunk.map())?;

        let content = take_till(|c: char| c == '\r' || c == '\n')
            .parse(input)
            .map_err(Kind::IdentationNonblankChunk.map())?;

        if content.is_empty() {
            return Err(MarkDownError::Kind(
                Kind::IdentationNonblankChunk,
                ControlFlow::Recovable,
                Span::Range(input.start()..cmp::min(input.start() + 2, input.end())),
            ));
        };

        Ok(Self {
            identation,
            content,
            line_ending: input.parse().map_err(Kind::IdentationNonblankChunk.map())?,
        })
    }
    #[inline]
    fn to_span(&self) -> parserc::Span {
        self.identation
            .to_span()
            .union(&self.content.to_span())
            .union(&self.line_ending.to_span())
    }
}

/// Blank lines, each preceded by four or more spaces of indentation.
///
/// See [`https://spec.commonmark.org/0.31.2/#fenced-code-blocks`]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[syntax(map_err = Kind::IdentationNonblankChunk.map())]
pub struct IndentedBlankLine<I>
where
    I: MarkDownInput,
{
    /// optional leading whitespaces.
    pub leading_whitespaces: S<I>,
    /// Required line ending characters.
    pub line_ending: LineEnding<I>,
}

/// [`indented chunk`] is a sequence of non-blank lines, each preceded by four or more spaces of indentation.
///
/// [`indented chunk`]: https://spec.commonmark.org/0.31.2/#indented-chunk
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IdentedChunk<I>
where
    I: MarkDownInput,
{
    NonBlank(IndentedNonblankLine<I>),
    Blank(IndentedBlankLine<I>),
}

///An [`indented code block`] is composed of one or more indented chunks separated by blank lines.
///
/// [`indented code block`]: https://spec.commonmark.org/0.31.2/#indented-code-block
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Syntax)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndentedCodeBlock<I>(pub Vec<IdentedChunk<I>>)
where
    I: MarkDownInput;

#[cfg(test)]
mod tests {
    use parserc::{ControlFlow, Span, syntax::InputSyntaxExt};

    use crate::{
        IdentedChunk, IndentationFrom, IndentedBlankLine, IndentedCodeBlock, IndentedNonblankLine,
        Kind, LineEnding, MarkDownError, S, TokenStream,
    };

    #[test]
    fn test_blank_line() {
        assert_eq!(
            TokenStream::from("\r\n").parse(),
            Ok(IndentedBlankLine {
                leading_whitespaces: S(TokenStream::from("")),
                line_ending: LineEnding::CrLf(TokenStream::from("\r\n"))
            })
        );

        assert_eq!(
            TokenStream::from(" \r\n").parse(),
            Ok(IndentedBlankLine {
                leading_whitespaces: S(TokenStream::from(" ")),
                line_ending: LineEnding::CrLf(TokenStream::from((1, "\r\n")))
            })
        );

        assert_eq!(
            TokenStream::from("hello\r\n").parse::<IndentedBlankLine<_>>(),
            Err(MarkDownError::Kind(
                Kind::IdentationNonblankChunk,
                ControlFlow::Recovable,
                Span::Range(0..1)
            ))
        );
    }

    #[test]
    fn test_nonblank_line() {
        assert_eq!(
            TokenStream::from("      helle world").parse(),
            Ok(IndentedNonblankLine {
                identation: IndentationFrom(TokenStream::from("      ")),
                content: TokenStream::from((6, "helle world")),
                line_ending: None,
            })
        );

        assert_eq!(
            TokenStream::from("    helle world\n").parse(),
            Ok(IndentedNonblankLine {
                identation: IndentationFrom(TokenStream::from("    ")),
                content: TokenStream::from((4, "helle world")),
                line_ending: Some(LineEnding::LF(TokenStream::from((15, "\n")))),
            })
        );

        assert_eq!(
            TokenStream::from("      \r\n").parse::<IndentedNonblankLine<_>>(),
            Err(MarkDownError::Kind(
                Kind::IdentationNonblankChunk,
                ControlFlow::Recovable,
                Span::Range(6..8)
            ))
        );
    }

    #[test]
    fn test_code_block() {
        assert_eq!(
            TokenStream::from("     helle world\n").parse(),
            Ok(IndentedCodeBlock(vec![IdentedChunk::NonBlank(
                IndentedNonblankLine {
                    identation: IndentationFrom(TokenStream::from("     ")),
                    content: TokenStream::from((5, "helle world")),
                    line_ending: Some(LineEnding::LF(TokenStream::from((16, "\n"))))
                }
            )]))
        );

        assert_eq!(
            TokenStream::from("     helle\n\n   \n    world").parse(),
            Ok(IndentedCodeBlock(vec![
                IdentedChunk::NonBlank(IndentedNonblankLine {
                    identation: IndentationFrom(TokenStream::from("     ")),
                    content: TokenStream::from((5, "helle")),
                    line_ending: Some(LineEnding::LF(TokenStream::from((10, "\n"))))
                }),
                IdentedChunk::Blank(IndentedBlankLine {
                    leading_whitespaces: S(TokenStream::from((11, ""))),
                    line_ending: LineEnding::LF(TokenStream::from((11, "\n")))
                }),
                IdentedChunk::Blank(IndentedBlankLine {
                    leading_whitespaces: S(TokenStream::from((12, "   "))),
                    line_ending: LineEnding::LF(TokenStream::from((15, "\n")))
                }),
                IdentedChunk::NonBlank(IndentedNonblankLine {
                    identation: IndentationFrom(TokenStream::from((16, "    "))),
                    content: TokenStream::from((20, "world")),
                    line_ending: None
                }),
            ])),
        );
    }
}
