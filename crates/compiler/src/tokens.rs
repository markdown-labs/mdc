use parserc::{ControlFlow, Item, ParseError, Parser, next_if, syntax::Syntax, take_while};

use crate::{MarkDownError, MarkDownInput};

mod kw {
    use parserc::syntax::keyword;

    keyword!(LR, "\n");
    keyword!(CRLR, "\r\n");
}

/// Syntax for newline token.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NewLine<I>
where
    I: MarkDownInput,
{
    /// \n
    LR(I),
    /// \r\n
    CRLR(I),
}

impl<I> Syntax<I> for NewLine<I>
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
}

// Whitespace chars.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct S<I>(pub I)
where
    I: MarkDownInput;

impl<I> Syntax<I> for S<I>
where
    I: MarkDownInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        take_while(|c: char| c != '\r' && c != '\n' && c.is_whitespace())
            .parse(input)
            .map(|input| Self(input))
    }
}

#[cfg(test)]
mod tests {
    use parserc::Span;

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
            NewLine::parse(&mut input),
            Ok(NewLine::LR(TokenStream::from("\n")))
        );

        let mut input = TokenStream::from("\r\n");

        assert_eq!(
            NewLine::parse(&mut input),
            Ok(NewLine::CRLR(TokenStream::from("\r\n")))
        );

        let mut input = TokenStream::from("\r \n");

        assert_eq!(
            NewLine::parse(&mut input),
            Err(MarkDownError::NewLine(
                ControlFlow::Recovable,
                Span::Range(0..3)
            ))
        );
    }

    #[test]
    fn test_s() {
        let mut input = TokenStream::from("     ");

        assert_eq!(S::parse(&mut input), Ok(S(TokenStream::from("     "))));
    }
}
