use parserc::{
    ControlFlow, ParseError, Parser,
    syntax::{LimitsTo, Syntax, keyword},
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

/// The leading whitespaces for one line.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentWhiteSpaces<I, const N: usize>(pub I)
where
    I: MarkDownInput;

impl<I, const N: usize> Syntax<I> for IdentWhiteSpaces<I, N>
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

impl<I> Syntax<I> for LineEnding<I>
where
    I: MarkDownInput,
{
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        keyword!(LF, "\n");
        keyword!(CrLf, "\r\n");

        Lf::into_parser()
            .map(|input| LineEnding::LF(input.0))
            .or(CrLf::into_parser().map(|input| Self::CrLf(input.0)))
            .parse(input)
            .map_err(|err| MarkDownError::Kind(Kind::LineEnding, err.control_flow(), err.span()))
    }

    fn to_span(&self) -> parserc::Span {
        match self {
            LineEnding::LF(input) => input.to_span(),
            LineEnding::CrLf(input) => input.to_span(),
        }
    }
}
