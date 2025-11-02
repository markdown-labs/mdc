use std::{cmp, collections::HashSet, sync::LazyLock};

use entities::ENTITIES;
use parserc::{ControlFlow, ParseError, Parser, Span, next, syntax::Syntax};

use crate::{Kind, MarkDownError, MarkDownInput};

/// Valid entity names.
#[allow(unused)]
static NAMES: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| ENTITIES.iter().map(|entity| entity.entity).collect());

static MAX_ENTITY_LEN: usize = 100;

/// HTML5 entity characters
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Entity<I>(pub I)
where
    I: MarkDownInput;

impl<I> Syntax<I> for Entity<I>
where
    I: MarkDownInput,
{
    fn parse(input: &mut I) -> Result<Self, <I as parserc::Input>::Error> {
        next('&')
            .parse(&mut input.clone())
            .map_err(|err| MarkDownError::Kind(Kind::Entity, err.control_flow(), err.span()))?;

        let mut last = None;

        for (index, c) in input.iter_indices() {
            if index > MAX_ENTITY_LEN {
                break;
            }

            if c == ';' {
                last = Some(index + 1);
                break;
            }
        }

        let Some(last) = last else {
            let start = input.start();
            let span = Span::Range(start..start + cmp::min(100, input.len()));

            return Err(MarkDownError::Kind(Kind::Entity, ControlFlow::Fatal, span));
        };

        let content = input.split_to(last);

        if !NAMES.contains(content.as_str()) {
            return Err(MarkDownError::Kind(
                Kind::Entity,
                ControlFlow::Fatal,
                content.to_span(),
            ));
        }

        Ok(Self(content))
    }

    fn to_span(&self) -> parserc::Span {
        self.0.to_span()
    }
}

#[cfg(test)]
mod tests {
    use parserc::{ControlFlow, Span, syntax::InputSyntaxExt};

    use crate::{Entity, Kind, MarkDownError, TokenStream, entity::MAX_ENTITY_LEN};

    #[test]
    fn test_entities() {
        assert_eq!(
            TokenStream::from("&amp;").parse(),
            Ok(Entity(TokenStream::from("&amp;")))
        );

        let input = format!("&{};", "a".repeat(MAX_ENTITY_LEN));

        assert_eq!(
            TokenStream::from(input.as_str()).parse::<Entity<_>>(),
            Err(MarkDownError::Kind(
                Kind::Entity,
                ControlFlow::Fatal,
                Span::Range(0..100)
            ))
        );

        assert_eq!(
            TokenStream::from("&amp").parse::<Entity<_>>(),
            Err(MarkDownError::Kind(
                Kind::Entity,
                ControlFlow::Fatal,
                Span::Range(0..4)
            ))
        );

        assert_eq!(
            TokenStream::from("amp").parse::<Entity<_>>(),
            Err(MarkDownError::Kind(
                Kind::Entity,
                ControlFlow::Recovable,
                Span::Range(0..1)
            ))
        );

        assert_eq!(
            TokenStream::from("&abc;").parse::<Entity<_>>(),
            Err(MarkDownError::Kind(
                Kind::Entity,
                ControlFlow::Fatal,
                Span::Range(0..5)
            ))
        );
    }
}
