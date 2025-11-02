use parserc::{ControlFlow, Parser, next, syntax::Syntax};

use crate::{Kind, LineEnding, MarkDownError, MarkDownInput};

/// Escaped characters
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Escaped<I>
where
    I: MarkDownInput,
{
    /// escaped `*`
    Star(I),
    /// escaped `<`
    Lt(I),
    /// escaped `[`
    Square(I),
    /// escaped ```
    Backtick(I),
    /// escaped `.`
    Dot(I),
    /// escaped `#`
    Pound(I),
    /// escaped `&`
    And(I),
    /// escaped `\`
    Backslash(I),
    /// A backslash at the end of the line.
    HardlineBreak(I),
}

impl<I> Syntax<I> for Escaped<I>
where
    I: MarkDownInput,
{
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        let mut start = input.clone();

        next('\\').parse(input)?;

        match input.iter().next() {
            Some('*') => {
                input.split_to(1);
                Ok(Escaped::Star(start.split_to(2)))
            }
            Some('<') => {
                input.split_to(1);
                Ok(Escaped::Lt(start.split_to(2)))
            }
            Some('[') => {
                input.split_to(1);
                Ok(Escaped::Square(start.split_to(2)))
            }
            Some('`') => {
                input.split_to(1);
                Ok(Escaped::Backtick(start.split_to(2)))
            }
            Some('.') => {
                input.split_to(1);
                Ok(Escaped::Dot(start.split_to(2)))
            }
            Some('#') => {
                input.split_to(1);
                Ok(Escaped::Pound(start.split_to(2)))
            }
            Some('&') => {
                input.split_to(1);
                Ok(Escaped::And(start.split_to(2)))
            }
            Some('\\') => {
                input.split_to(1);
                Ok(Escaped::Backslash(start.split_to(2)))
            }
            _ => {
                if let Some(line_ending) = LineEnding::into_parser().ok().parse(input)? {
                    let len = match line_ending {
                        LineEnding::LF(_) => 1,
                        LineEnding::CrLf(_) => 2,
                    };
                    return Ok(Escaped::HardlineBreak(start.split_to(len + 1)));
                }

                return Err(MarkDownError::Kind(
                    Kind::Escaped,
                    ControlFlow::Recovable,
                    start.to_span(),
                ));
            }
        }
    }

    #[inline]
    fn to_span(&self) -> parserc::Span {
        match self {
            Escaped::Star(input) => input.to_span(),
            Escaped::Lt(input) => input.to_span(),
            Escaped::Square(input) => input.to_span(),
            Escaped::Backtick(input) => input.to_span(),
            Escaped::Dot(input) => input.to_span(),
            Escaped::Pound(input) => input.to_span(),
            Escaped::And(input) => input.to_span(),
            Escaped::Backslash(input) => input.to_span(),
            Escaped::HardlineBreak(input) => input.to_span(),
        }
    }
}

#[cfg(test)]
mod tests {
    use parserc::syntax::InputSyntaxExt;

    use crate::{Escaped, TokenStream};

    #[test]
    fn test_escaped() {
        assert_eq!(
            TokenStream::from(r#"\["#).parse(),
            Ok(Escaped::Square(TokenStream::from(r#"\["#)))
        );

        assert_eq!(
            TokenStream::from(r#"\*"#).parse(),
            Ok(Escaped::Star(TokenStream::from(r#"\*"#)))
        );

        assert_eq!(
            TokenStream::from(r#"\<"#).parse(),
            Ok(Escaped::Lt(TokenStream::from(r#"\<"#)))
        );

        assert_eq!(
            TokenStream::from(r#"\`"#).parse(),
            Ok(Escaped::Backtick(TokenStream::from(r#"\`"#)))
        );

        assert_eq!(
            TokenStream::from(r#"\\"#).parse(),
            Ok(Escaped::Backslash(TokenStream::from(r#"\\"#)))
        );

        assert_eq!(
            TokenStream::from(r#"\#"#).parse(),
            Ok(Escaped::Pound(TokenStream::from(r#"\#"#)))
        );

        assert_eq!(
            TokenStream::from(r#"\."#).parse(),
            Ok(Escaped::Dot(TokenStream::from(r#"\."#)))
        );

        assert_eq!(
            TokenStream::from(r#"\&"#).parse(),
            Ok(Escaped::And(TokenStream::from(r#"\&"#)))
        );

        assert_eq!(
            TokenStream::from("\\\n").parse(),
            Ok(Escaped::HardlineBreak(TokenStream::from("\\\n")))
        );

        assert_eq!(
            TokenStream::from("\\\r\n").parse(),
            Ok(Escaped::HardlineBreak(TokenStream::from("\\\r\n")))
        );
    }
}
