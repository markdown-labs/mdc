use parserc::{
    ControlFlow, ParseError, Parser,
    syntax::{LimitsFrom, LimitsTo, Syntax, keyword},
    take_while,
};

use crate::{Kind, MarkDownError, MarkDownInput};

/// Whitespace chars.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct S<I>(pub I)
where
    I: MarkDownInput;

impl<I> Syntax<I> for S<I>
where
    I: MarkDownInput,
{
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        take_while(|c: char| c != '\n' && c != '\r' && c.is_whitespace())
            .parse(input)
            .map(|c| Self(c))
    }

    fn to_span(&self) -> parserc::Span {
        self.0.to_span()
    }
}

/// Non-empty whitespace chars.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct S1<I>(pub I)
where
    I: MarkDownInput;

impl<I> Syntax<I> for S1<I>
where
    I: MarkDownInput,
{
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let content =
            take_while(|c: char| c != '\n' && c != '\r' && c.is_whitespace()).parse(input)?;

        if content.is_empty() {
            return Err(MarkDownError::Kind(
                Kind::S,
                ControlFlow::Recovable,
                content.to_span(),
            ));
        }

        Ok(Self(content))
    }

    fn to_span(&self) -> parserc::Span {
        self.0.to_span()
    }
}

/// Up to `N` spaces of indentation.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndentationTo<I, const N: usize>(pub I)
where
    I: MarkDownInput;

impl<I, const N: usize> Syntax<I> for IndentationTo<I, N>
where
    I: MarkDownInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        LimitsTo::<S<I>, N>::parse(input)
            .map_err(|err| {
                MarkDownError::Kind(Kind::LeadingWhiteSpace, err.control_flow(), err.span())
            })
            .map(|content| Self(content.0.0))
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        self.0.to_span()
    }
}

/// Preceded by `N` or more spaces of indentation
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndentationFrom<I, const N: usize>(pub I)
where
    I: MarkDownInput;

impl<I, const N: usize> Syntax<I> for IndentationFrom<I, N>
where
    I: MarkDownInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        LimitsFrom::<S<I>, N>::parse(input)
            .map_err(|err| {
                MarkDownError::Kind(Kind::LeadingWhiteSpace, err.control_flow(), err.span())
            })
            .map(|content| Self(content.0.0))
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        self.0.to_span()
    }
}

/// Line ending characters.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LineEnding<I>
where
    I: MarkDownInput,
{
    LF(I),
    CrLf(I),
}

impl<I> LineEnding<I>
where
    I: MarkDownInput,
{
    /// Convert `LineEnding` into `MarkDownInput`
    #[inline]
    pub(crate) fn into_input(self) -> I {
        match self {
            LineEnding::LF(input) => input,
            LineEnding::CrLf(input) => input,
        }
    }
}

impl<I> Syntax<I> for LineEnding<I>
where
    I: MarkDownInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        keyword!(LF, "\n");
        keyword!(CrLf, "\r\n");

        Lf::into_parser()
            .map(|input| LineEnding::LF(input.0))
            .or(CrLf::into_parser().map(|input| Self::CrLf(input.0)))
            .parse(input)
            .map_err(|err| MarkDownError::Kind(Kind::LineEnding, err.control_flow(), err.span()))
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        match self {
            LineEnding::LF(input) => input.to_span(),
            LineEnding::CrLf(input) => input.to_span(),
        }
    }
}

/// A blank line contains no characters other than the line ending characters.
///
/// See [`https://spec.commonmark.org/0.31.2/#blank-lines`]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlankLine<I>(pub I)
where
    I: MarkDownInput;

impl<I> Syntax<I> for BlankLine<I>
where
    I: MarkDownInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        LineEnding::parse(input)
            .map(|content| Self(content.into_input()))
            .map_err(Kind::BlankLine.map())
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        self.0.to_span()
    }
}
