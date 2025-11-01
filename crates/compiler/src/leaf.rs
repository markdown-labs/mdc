use parserc::{
    ParseError, Parser,
    syntax::{LimitsTo, Punctuated, Syntax, token},
    take_while,
};

use crate::{MarkDownError, MarkDownInput};

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
        take_while(|c: char| c.is_whitespace())
            .map(|content| Self(content))
            .parse(input)
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
            .map_err(|err| MarkDownError::LeadingWhiteSpace(err.control_flow(), err.span()))
            .map(|content| Self(content.0.0))
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        self.0.to_span()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum ThematicChars<I>
where
    I: MarkDownInput,
{
    Stars(I),
    Underscores(I),
    Minus(I),
}

impl<I> ThematicChars<I>
where
    I: MarkDownInput,
{
    fn value(&self) -> usize {
        match self {
            ThematicChars::Stars(_) => 1,
            ThematicChars::Underscores(_) => 2,
            ThematicChars::Minus(_) => 3,
        }
    }
}

impl<I> Syntax<I> for ThematicChars<I>
where
    I: MarkDownInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        token!(Stars, |c: char| c == '*');
        token!(Underscores, |c: char| c == '_');
        token!(Minus, |c: char| c == '-');

        Stars::into_parser()
            .map(|v| Self::Stars(v.0))
            .or(Underscores::into_parser().map(|v| Self::Underscores(v.0)))
            .or(Minus::into_parser().map(|v| Self::Minus(v.0)))
            .parse(input)
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        match self {
            ThematicChars::Stars(content) => content.to_span(),
            ThematicChars::Underscores(content) => content.to_span(),
            ThematicChars::Minus(content) => content.to_span(),
        }
    }
}

/// Leaf block: [`thematic breaks`]
///
/// [`thematic breaks`]: https://spec.commonmark.org/0.31.2/#thematic-break
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThematicBreaks<I>
where
    I: MarkDownInput,
{
    /// A line consisting of optionally up to three spaces of indentation
    ident_whitespaces: IdentWhiteSpaces<I, 3>,
    breaks: Punctuated<ThematicChars<I>, S<I>>,
}

impl<I> Syntax<I> for ThematicBreaks<I>
where
    I: MarkDownInput,
{
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let ident_whitespaces = IdentWhiteSpaces::<I, 3>::parse(input)?;

        Ok(Self {
            ident_whitespaces,
            breaks: todo!(),
        })
    }

    fn to_span(&self) -> parserc::Span {
        todo!()
    }
}
