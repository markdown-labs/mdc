use parserc::{
    Parser,
    syntax::{InputSyntaxExt, Limits, Syntax, token},
    take_while,
};

use crate::{IdentWhiteSpaces, Kind, LineEnding, MarkDownInput};

/// An [`ATX heading`] parser.
///
/// [`ATX heading`]: https://spec.commonmark.org/0.31.2/#atx-heading
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ATXHeading<I>
where
    I: MarkDownInput,
{
    /// Up to three spaces of indentation are allowed:
    ident_whitespaces: IdentWhiteSpaces<I, 3>,
    /// keywords `##...`.
    leading_pounds: I,
    /// heading content.
    content: I,
    /// Optional line ending chars.
    line_ending: Option<LineEnding<I>>,
}

impl<I> Syntax<I> for ATXHeading<I>
where
    I: MarkDownInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let ident_whitespaces =
            IdentWhiteSpaces::<I, 3>::parse(input).map_err(Kind::ATXHeading.map())?;

        token!(Pounds, |c: char| c == '#');

        let leading_pounds =
            Limits::<Pounds<_>, 1, 7>::parse(input).map_err(Kind::ATXHeading.map())?;

        let content = take_while(|c: char| c != '\r' && c != '\n').parse(input)?;

        let line_ending = input.parse().map_err(Kind::ATXHeading.map())?;

        Ok(Self {
            ident_whitespaces,
            leading_pounds: leading_pounds.0.0,
            content,
            line_ending,
        })
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        self.ident_whitespaces
            .to_span()
            .union(&self.leading_pounds.to_span())
            .union(&self.content.to_span())
            .union(&self.line_ending.to_span())
    }
}

#[cfg(test)]
mod tests {
    use parserc::syntax::InputSyntaxExt;

    use crate::{ATXHeading, IdentWhiteSpaces, LineEnding, TokenStream};

    #[test]
    fn test_atx_heading() {
        assert_eq!(
            TokenStream::from(" ###### hello world\r\n").parse(),
            Ok(ATXHeading {
                ident_whitespaces: IdentWhiteSpaces(TokenStream::from(" ")),
                leading_pounds: TokenStream::from((1, "######")),
                content: TokenStream::from((7, " hello world")),
                line_ending: Some(LineEnding::CrLf(TokenStream::from((19, "\r\n"))))
            })
        );

        assert_eq!(
            TokenStream::from("###### hello world ").parse(),
            Ok(ATXHeading {
                ident_whitespaces: IdentWhiteSpaces(TokenStream::from("")),
                leading_pounds: TokenStream::from("######"),
                content: TokenStream::from((6, " hello world ")),
                line_ending: None
            })
        );

        assert_eq!(
            TokenStream::from("   #").parse(),
            Ok(ATXHeading {
                ident_whitespaces: IdentWhiteSpaces(TokenStream::from("   ")),
                leading_pounds: TokenStream::from((3, "#")),
                content: TokenStream::from((4, "")),
                line_ending: None
            })
        );
    }
}
