use parserc::{
    ControlFlow, Item, ParseError, Parser, next_if,
    syntax::{LimitsTo, Syntax, token},
    take_while,
};

use crate::{MarkDownError, MarkDownInput};

mod kw {
    use parserc::syntax::keyword;

    keyword!(LR, "\n");
    keyword!(CRLR, "\r\n");
}

/// Token for line ending.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LineEnding<I>
where
    I: MarkDownInput,
{
    /// \n
    LR(I),
    /// \r\n
    CRLR(I),
}

impl<I> Syntax<I> for LineEnding<I>
where
    I: MarkDownInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        kw::Crlr::into_parser()
            .map(|input| Self::CRLR(input.0))
            .or(kw::Lr::into_parser().map(|input| Self::LR(input.0)))
            .parse(input)
            .map_err(|err| MarkDownError::NewLine(err.control_flow(), err.span()))
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        match self {
            LineEnding::LR(v) => v.to_span(),
            LineEnding::CRLR(v) => v.to_span(),
        }
    }
}

/// Thematic break chars.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThematicBreaks<I>
where
    I: MarkDownInput,
{
    Stars(I),
    Underscores(I),
    Minus(I),
}

impl<I> Syntax<I> for ThematicBreaks<I>
where
    I: MarkDownInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let mut content = input.clone();

        let next = next_if(|next| next == '*' || next == '_' || next == '-').parse(input)?;

        let next = next.iter().next().unwrap();

        let tails = take_while(|c| c == next).parse(input)?;

        content.split_off(next.len() + tails.len());

        if content.len() < 3 {
            return Err(MarkDownError::Horizon(
                ControlFlow::Recovable,
                content.to_span(),
            ));
        }

        match next {
            '*' => Ok(Self::Stars(content)),
            '_' => Ok(Self::Underscores(content)),
            '-' => Ok(Self::Minus(content)),
            _ => unreachable!("Safety: guard by `next_if ...`"),
        }
    }

    fn to_span(&self) -> parserc::Span {
        match self {
            ThematicBreaks::Stars(v) => v.to_span(),
            ThematicBreaks::Underscores(v) => v.to_span(),
            ThematicBreaks::Minus(v) => v.to_span(),
        }
    }
}

token!(S, |c: char| c != '\r' && c != '\n' && c.is_whitespace());

// Block leading whitespaces.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LeadingWhiteSpace<I, const N: usize>(pub I)
where
    I: MarkDownInput;

impl<I, const N: usize> Syntax<I> for LeadingWhiteSpace<I, N>
where
    I: MarkDownInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let s = LimitsTo::<S<I>, N>::parse(input)
            .map_err(|err| MarkDownError::LeadingWhiteSpace(err.control_flow(), err.span()))?;

        Ok(Self(s.0.0))
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        self.0.to_span()
    }
}

//Leading pound chars for `headings` block.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LeadingPounds<I>(pub I);

impl<I> Syntax<I> for LeadingPounds<I>
where
    I: MarkDownInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let content = take_while(|c| c == '#').parse(input)?;

        let len = match content.to_span() {
            parserc::Span::None => 0,
            parserc::Span::Range(range) => range.len(),
            parserc::Span::RangeTo(range_to) => range_to.end,
            _ => {
                return Err(MarkDownError::LeadingPounds(
                    ControlFlow::Recovable,
                    content.to_span(),
                ));
            }
        };

        if len < 1 || !(len < 6) {
            return Err(MarkDownError::LeadingPounds(
                ControlFlow::Recovable,
                content.to_span(),
            ));
        }

        Ok(Self(content))
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        self.0.to_span()
    }
}

#[cfg(test)]
mod tests {
    use parserc::{Span, syntax::InputSyntaxExt};

    use super::*;
    use crate::TokenStream;

    #[test]
    fn test_horizon() {
        let mut input = TokenStream::from("***");

        assert_eq!(
            ThematicBreaks::parse(&mut input),
            Ok(ThematicBreaks::Stars(TokenStream::from("***")))
        );

        let mut input = TokenStream::from("**");

        assert_eq!(
            ThematicBreaks::parse(&mut input),
            Err(MarkDownError::Horizon(
                ControlFlow::Recovable,
                Span::Range(0..2)
            ))
        );
    }

    #[test]
    fn test_newline() {
        let mut input = TokenStream::from("\n");

        assert_eq!(
            LineEnding::parse(&mut input),
            Ok(LineEnding::LR(TokenStream::from("\n")))
        );

        let mut input = TokenStream::from("\r\n");

        assert_eq!(
            LineEnding::parse(&mut input),
            Ok(LineEnding::CRLR(TokenStream::from("\r\n")))
        );

        let mut input = TokenStream::from("\r \n");

        assert_eq!(
            LineEnding::parse(&mut input),
            Err(MarkDownError::NewLine(
                ControlFlow::Recovable,
                Span::Range(0..3)
            ))
        );
    }

    #[test]
    fn test_s() {
        let mut input = TokenStream::from("     \t");

        assert_eq!(S::parse(&mut input), Ok(S(TokenStream::from("     \t"))));
    }

    #[test]
    fn test_leading_pounds() {
        for i in 1..6 {
            let code = "#".repeat(i);

            assert_eq!(
                TokenStream::from(code.as_str()).parse(),
                Ok(LeadingPounds(TokenStream::from(code.as_str())))
            );
        }

        assert_eq!(
            TokenStream::from("  ").parse::<LeadingPounds<_>>(),
            Err(MarkDownError::LeadingPounds(
                ControlFlow::Recovable,
                Span::Range(0..0),
            ))
        );

        assert_eq!(
            TokenStream::from("#######").parse::<LeadingPounds<_>>(),
            Err(MarkDownError::LeadingPounds(
                ControlFlow::Recovable,
                Span::Range(0..7),
            ))
        );
    }
}
