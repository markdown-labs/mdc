use parserc::{
    ControlFlow, Item, Parser, next_if,
    syntax::{Syntax, keyword},
    take_while,
};

use crate::{MarkDownError, MarkDownInput};

keyword!(LR, "\n");
keyword!(CRLR, "\r\n");

/// Syntax for newline token.
#[derive(Debug, Clone, PartialEq, Eq, Syntax)]
pub enum NewLine<I>
where
    I: MarkDownInput,
{
    /// \n
    LR(Lr<I>),
    /// \r\n
    CRLR(Crlr<I>),
}

/// Valid horizon chars.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Horizon<I>
where
    I: MarkDownInput,
{
    Stars(I),
    Underscores(I),
    Minus(I),
}

impl<I> Syntax<I> for Horizon<I>
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
#[derive(Debug, Clone)]
pub struct S<I>(I)
where
    I: MarkDownInput;

impl<I> Syntax<I> for S<I>
where
    I: MarkDownInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        take_while(|c: char| c.is_whitespace())
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
            Horizon::parse(&mut input),
            Ok(Horizon::Stars(TokenStream::from("***")))
        );

        let mut input = TokenStream::from("**");

        assert_eq!(
            Horizon::parse(&mut input),
            Err(MarkDownError::Horizon(
                ControlFlow::Recovable,
                Span::Range(0..2)
            ))
        );
    }
}
