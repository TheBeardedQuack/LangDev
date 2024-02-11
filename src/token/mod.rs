mod keyword;
pub use keyword::*;

mod identifier;
pub use identifier::*;

mod number;
pub use number::*;

mod symbol;
pub use symbol::*;

use crate::error::TokenParseError;

#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub struct Token<'a>
{
    kind: TokenKind,
    span: &'a str,
}

#[derive(Clone, Debug)]
#[derive(PartialEq, Hash)]
pub enum TokenKind
{
    Identifier(Identifier),
    Keyword(Keyword),
    Number(Number),
    Symbol(Symbol),
}

impl<'a> Token<'a>
{
    #[inline]
    pub(crate) fn end_of_input(
        end_of_text: &'a str
    ) -> Self {
        Self{
            kind: TokenKind::Symbol(Symbol::EndOfInput),
            span: end_of_text,
        }
    }

    pub fn kind(
        &self
    ) -> &TokenKind {
        &self.kind
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(
        &self
    ) -> usize {
        self.span.len()
    }
}

impl<'me, 's: 'me> TryFrom<&'s str>
for Token<'me>
{
    type Error = TokenParseError;

    fn try_from(
        input: &'s str
    ) -> Result<Self, Self::Error> {
        if let Ok(ident) = input.parse::<Identifier>() {
            match Keyword::try_from(&ident) {
                Ok(kw) => {  
                    let len = kw.len();
                    Ok(Self{
                        kind: TokenKind::Keyword(kw),
                        span: &input[0..len],
                    })
                },
                Err(TokenParseError::InvalidKeyword) => {
                    let len = ident.len();
                    Ok(Self{
                        kind: TokenKind::Identifier(ident),
                        span: &input[0..len],
                    })
                }
                Err(err) => Err(err)?,
            }
        }
        else if let Ok(num) = input.parse::<Number>() {
            Ok(Self{
                kind: TokenKind::Number(num),
                span: &input[0..num.len()],
            })
        }
        else if let Ok(sym) = input.parse::<Symbol>() {
            Ok(Self{
                kind: TokenKind::Symbol(sym),
                span: &input[0..sym.len()],
            })
        }
        else {
            Err(TokenParseError::InvalidToken)
        }
    }
}

impl AsRef<str>
for Token<'_>
{
    fn as_ref(
        &self
    ) -> &str {
        self.span
    }
}
