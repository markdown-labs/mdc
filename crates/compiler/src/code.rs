use std::cmp;

use parserc::{
    ControlFlow, Parser, Span,
    syntax::{InputSyntaxExt, Syntax},
    take_till,
};

use crate::{IndentationFrom, Kind, LineEnding, MarkDownError, MarkDownInput};

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
    /// Preceded by four or more spaces of indentation.
    pub identation: Option<IndentationFrom<I, 4>>,
    /// Required line ending characters.
    pub line_ending: LineEnding<I>,
}

#[cfg(test)]
mod tests {
    use parserc::{ControlFlow, Span, syntax::InputSyntaxExt};

    use crate::{
        IndentationFrom, IndentedBlankLine, IndentedNonblankLine, Kind, LineEnding, MarkDownError,
        TokenStream,
    };

    #[test]
    fn test_blank_line() {
        assert_eq!(
            TokenStream::from("\r\n").parse(),
            Ok(IndentedBlankLine {
                identation: None,
                line_ending: LineEnding::CrLf(TokenStream::from("\r\n"))
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
}
