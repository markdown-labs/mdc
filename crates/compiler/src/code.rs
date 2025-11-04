use std::cmp;

use parserc::{
    ControlFlow, Parser, Span, next_if,
    syntax::{InputSyntaxExt, Syntax},
    take_till, take_until, take_while,
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

/// Non-empty backtick characters (`) or tildes (~).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fenced<I>(pub I)
where
    I: MarkDownInput;

impl<I> Syntax<I> for Fenced<I>
where
    I: MarkDownInput,
{
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let mut content = input.clone();

        let start = next_if(|c: char| c == '`' || c == '~').parse(input)?;

        let next = match start.as_str() {
            "`" => '`',
            "~" => '~',
            _ => unreachable!(""),
        };

        let trailing = take_while(|c: char| c == next).parse(input)?;

        *input = content.split_off(trailing.len() + 1);

        if content.len() < 3 {
            return Err(MarkDownError::Kind(
                Kind::FencedCodeBlock,
                ControlFlow::Recovable,
                content.to_span(),
            ));
        }

        Ok(Self(content))
    }

    fn to_span(&self) -> Span {
        self.0.to_span()
    }
}

/// A [`code fence`] is a sequence of at least three consecutive backtick characters (`) or tildes (~).
///
/// [`code fence`]: https://spec.commonmark.org/0.31.2/#code-fence
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FencedCodeBlock<I>
where
    I: MarkDownInput,
{
    /// Start tag.
    pub start: Fenced<I>,
    /// Code body.
    pub body: I,
    /// End tag.
    pub end: Option<Fenced<I>>,
}

impl<I> Syntax<I> for FencedCodeBlock<I>
where
    I: MarkDownInput + 'static,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let start = Fenced::parse(input)?;

        let body = take_until(start.0.clone()).ok().parse(input)?;

        if let Some(body) = body {
            let end = Fenced::parse(input)?;

            assert_eq!(start.0.len(), end.0.len());

            Ok(Self {
                start,
                body,
                end: Some(end),
            })
        } else {
            Ok(Self {
                start,
                body: input.split_off(0),
                end: None,
            })
        }
    }

    #[inline]
    fn to_span(&self) -> Span {
        self.start
            .to_span()
            .union(&self.body.to_span())
            .union(&self.end.to_span())
    }
}

#[cfg(test)]
mod tests {
    use parserc::{ControlFlow, Span, syntax::InputSyntaxExt};

    use crate::{
        Fenced, FencedCodeBlock, IdentedChunk, IndentationFrom, IndentedBlankLine,
        IndentedCodeBlock, IndentedNonblankLine, Kind, LineEnding, MarkDownError, S, TokenStream,
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

        assert_eq!(
            TokenStream::from("     helle\n\n   \n    world\nworld").parse(),
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
                    line_ending: Some(LineEnding::LF(TokenStream::from((25, "\n"))))
                }),
            ])),
        );
    }

    #[test]
    fn test_fenced_code_block() {
        assert_eq!(
            TokenStream::from("~~~~\naaa\n~~~\n~~~~",).parse(),
            Ok(FencedCodeBlock {
                start: Fenced(TokenStream::from("~~~~")),
                body: TokenStream::from((4, "\naaa\n~~~\n")),
                end: Some(Fenced(TokenStream::from((13, "~~~~"))))
            })
        );

        assert_eq!(
            TokenStream::from("~~~~\naaa\n~~~\n",).parse(),
            Ok(FencedCodeBlock {
                start: Fenced(TokenStream::from("~~~~")),
                body: TokenStream::from((4, "\naaa\n~~~\n")),
                end: None
            })
        );
    }
}
