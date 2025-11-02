use parserc::{
    ControlFlow, Parser,
    syntax::{InputSyntaxExt, Limits, Syntax, token},
    take_while,
};

use crate::{IndentationTo, Kind, LineEnding, MarkDownError, MarkDownInput};

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
    pub ident_whitespaces: IndentationTo<I, 3>,
    /// keywords `##...`.
    pub leading_pounds: I,
    /// seperate spaces/tabs/line ending.
    pub seperate: I,
    /// heading content.
    pub content: I,
    /// Optional line ending chars.
    pub line_ending: Option<LineEnding<I>>,
}

impl<I> Syntax<I> for ATXHeading<I>
where
    I: MarkDownInput,
{
    #[inline]
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let ident_whitespaces =
            IndentationTo::<I, 3>::parse(input).map_err(Kind::ATXHeading.map())?;

        token!(Pounds, |c: char| c == '#');

        let leading_pounds =
            Limits::<Pounds<_>, 1, 7>::parse(input).map_err(Kind::ATXHeading.map())?;

        let mut content = take_while(|c: char| c != '\r' && c != '\n').parse(input)?;

        let line_ending: Option<LineEnding<_>> = input.parse().map_err(Kind::ATXHeading.map())?;

        let seperate = take_while(|c: char| c.is_whitespace()).parse(&mut content)?;

        if seperate.is_empty() && line_ending.is_none() {
            return Err(MarkDownError::Kind(
                Kind::ATXHeading,
                ControlFlow::Recovable,
                seperate.to_span(),
            ));
        }

        Ok(Self {
            ident_whitespaces,
            leading_pounds: leading_pounds.0.0,
            content,
            line_ending,
            seperate,
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
    use parserc::{ControlFlow, Span, syntax::InputSyntaxExt};

    use crate::{ATXHeading, IndentationTo, Kind, LineEnding, MarkDownError, TokenStream};

    #[test]
    fn test_atx_heading() {
        assert_eq!(
            TokenStream::from(" ###### hello world\r\n").parse(),
            Ok(ATXHeading {
                ident_whitespaces: IndentationTo(TokenStream::from(" ")),
                leading_pounds: TokenStream::from((1, "######")),
                seperate: TokenStream::from((7, " ")),
                content: TokenStream::from((8, "hello world")),
                line_ending: Some(LineEnding::CrLf(TokenStream::from((19, "\r\n"))))
            })
        );

        assert_eq!(
            TokenStream::from("###### hello world ").parse(),
            Ok(ATXHeading {
                ident_whitespaces: IndentationTo(TokenStream::from("")),
                leading_pounds: TokenStream::from("######"),
                seperate: TokenStream::from((6, " ")),
                content: TokenStream::from((7, "hello world ")),
                line_ending: None
            })
        );

        assert_eq!(
            TokenStream::from("   # ").parse(),
            Ok(ATXHeading {
                ident_whitespaces: IndentationTo(TokenStream::from("   ")),
                leading_pounds: TokenStream::from((3, "#")),
                seperate: TokenStream::from((4, " ")),
                content: TokenStream::from((5, "")),
                line_ending: None
            })
        );

        assert_eq!(
            TokenStream::from("   #").parse::<ATXHeading<_>>(),
            Err(MarkDownError::Kind(
                Kind::ATXHeading,
                ControlFlow::Recovable,
                Span::Range(4..4)
            ))
        );
    }
}
