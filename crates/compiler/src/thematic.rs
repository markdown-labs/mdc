use parserc::{
    ControlFlow, ParseError, Parser,
    syntax::{InputSyntaxExt, Punctuated, Syntax, token},
};

use crate::{Identation, IndentationTo, Kind, LineEnding, MarkDownError, MarkDownInput};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ThematicChars<I>
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

    #[inline]
    fn len(&self) -> usize {
        match self {
            ThematicChars::Stars(content) => content.len(),
            ThematicChars::Underscores(content) => content.len(),
            ThematicChars::Minus(content) => content.len(),
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
    pub ident_whitespaces: IndentationTo<I, 3>,
    /// core thematic breaks chars.
    pub breaks: Punctuated<ThematicChars<I>, Identation<I>>,
    /// optional line end.
    pub line_ending: Option<LineEnding<I>>,
}

impl<I> Syntax<I> for ThematicBreaks<I>
where
    I: MarkDownInput,
{
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let ident_whitespaces = IndentationTo::<I, 3>::parse(input)?;

        let breaks = Punctuated::<ThematicChars<I>, _>::parse(input)
            .map_err(|err| MarkDownError::Kind(Kind::Thematic, err.control_flow(), err.span()))?;

        let id = breaks.pairs.first().map(|pair| pair.0.value());

        let Some(id) = id else {
            if let Some(tail) = &breaks.tail {
                if tail.len() < 3 {
                    return Err(MarkDownError::Kind(
                        Kind::Thematic,
                        ControlFlow::Recovable,
                        tail.to_span(),
                    ));
                }

                let line_ending: Option<LineEnding<_>> = input.parse()?;

                if line_ending.is_none() && !input.is_empty() {
                    return Err(MarkDownError::Kind(
                        Kind::Thematic,
                        ControlFlow::Recovable,
                        breaks.to_span(),
                    ));
                }

                return Ok(Self {
                    ident_whitespaces,
                    breaks,
                    line_ending,
                });
            }

            return Err(MarkDownError::Kind(
                Kind::Thematic,
                ControlFlow::Recovable,
                ident_whitespaces.to_span(),
            ));
        };

        let mut len = 0;

        for (pair, _) in breaks.pairs.iter() {
            if pair.value() != id {
                return Err(MarkDownError::Kind(
                    Kind::Thematic,
                    ControlFlow::Recovable,
                    pair.to_span(),
                ));
            }

            len += pair.len();
        }

        if let Some(tail) = &breaks.tail {
            if tail.value() != id {
                return Err(MarkDownError::Kind(
                    Kind::Thematic,
                    ControlFlow::Recovable,
                    tail.to_span(),
                ));
            }

            len += tail.len();
        }

        if len < 3 {
            return Err(MarkDownError::Kind(
                Kind::Thematic,
                ControlFlow::Recovable,
                breaks.to_span(),
            ));
        }

        let line_ending: Option<LineEnding<_>> = input.parse()?;

        if line_ending.is_none() && !input.is_empty() {
            return Err(MarkDownError::Kind(
                Kind::Thematic,
                ControlFlow::Recovable,
                breaks.to_span(),
            ));
        }

        Ok(Self {
            ident_whitespaces,
            breaks,
            line_ending,
        })
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        self.ident_whitespaces
            .to_span()
            .union(&self.breaks.to_span())
    }
}

#[cfg(test)]
mod tests {
    use parserc::syntax::{InputSyntaxExt, Punctuated};

    use crate::TokenStream;

    use super::*;

    #[test]
    fn test_thematic_breaks() {
        assert_eq!(
            TokenStream::from("   -   -  -   ").parse(),
            Ok(ThematicBreaks {
                ident_whitespaces: IndentationTo(TokenStream::from("   ")),
                breaks: Punctuated {
                    pairs: vec![
                        (
                            ThematicChars::Minus(TokenStream::from((3, "-"))),
                            Identation(TokenStream::from((4, "   ")))
                        ),
                        (
                            ThematicChars::Minus(TokenStream::from((7, "-"))),
                            Identation(TokenStream::from((8, "  ")))
                        ),
                        (
                            ThematicChars::Minus(TokenStream::from((10, "-"))),
                            Identation(TokenStream::from((11, "   ")))
                        )
                    ],
                    tail: None
                },
                line_ending: None,
            })
        );

        assert_eq!(
            TokenStream::from("   -   -  -").parse(),
            Ok(ThematicBreaks {
                ident_whitespaces: IndentationTo(TokenStream::from("   ")),
                breaks: Punctuated {
                    pairs: vec![
                        (
                            ThematicChars::Minus(TokenStream::from((3, "-"))),
                            Identation(TokenStream::from((4, "   ")))
                        ),
                        (
                            ThematicChars::Minus(TokenStream::from((7, "-"))),
                            Identation(TokenStream::from((8, "  ")))
                        ),
                    ],
                    tail: Some(Box::new(ThematicChars::Minus(TokenStream::from((10, "-")))))
                },
                line_ending: None,
            })
        );

        assert_eq!(
            TokenStream::from("   -   -  -\n").parse(),
            Ok(ThematicBreaks {
                ident_whitespaces: IndentationTo(TokenStream::from("   ")),
                breaks: Punctuated {
                    pairs: vec![
                        (
                            ThematicChars::Minus(TokenStream::from((3, "-"))),
                            Identation(TokenStream::from((4, "   ")))
                        ),
                        (
                            ThematicChars::Minus(TokenStream::from((7, "-"))),
                            Identation(TokenStream::from((8, "  ")))
                        ),
                    ],
                    tail: Some(Box::new(ThematicChars::Minus(TokenStream::from((10, "-")))))
                },
                line_ending: Some(LineEnding::LF(TokenStream::from((11, "\n")))),
            })
        );
    }
}
