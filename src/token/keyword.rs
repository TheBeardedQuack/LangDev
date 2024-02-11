use super::{Identifier, TokenParseError};

use std::{fmt::Display, str::FromStr};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(EnumIter)]
pub enum Keyword
{
    Break,
    Const,
    Else,
    Enum,
    For,
    False,
    Function,
    If,
    Impl,
    Let,
    Match,
    Module,
    Mutable,
    Public,
    Return,
    Struct,
    Trait,
    True,
    Type,
    Use,
    While,
}

impl TryFrom<&Identifier>
for Keyword
{
    type Error = TokenParseError;

    fn try_from(
        value: &Identifier
    ) -> Result<Self, Self::Error> {
        match Self::iter().find(|kw| kw.as_str() == value.as_str()) {
            Some(kw) => Ok(kw),
            None => Err(TokenParseError::InvalidKeyword)
        }
    }
}

impl TryFrom<Identifier>
for Keyword
{
    type Error = TokenParseError;

    fn try_from(
        value: Identifier
    ) -> Result<Self, Self::Error> {
        match Self::iter().find(|kw| kw.as_str() == value.as_str()) {
            Some(kw) => Ok(kw),
            None => Err(TokenParseError::InvalidKeyword)
        }
    }
}

impl Keyword
{
    #[must_use]
    pub const fn len(
        &self
    ) -> usize {
        self.as_str().len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub const fn as_str(
        &self
    ) -> &'static str {
        match self {
            Self::Break     => "break",
            Self::Const     => "const",
            Self::Else      => "else",
            Self::Enum      => "enum",
            Self::False     => "false",
            Self::For       => "for",
            Self::Function  => "fn",
            Self::If        => "if",
            Self::Impl      => "impl",
            Self::Let       => "let",
            Self::Match     => "match",
            Self::Module    => "mod",
            Self::Mutable   => "mut",
            Self::Public    => "pub",
            Self::Return    => "return",
            Self::Struct    => "struct",
            Self::Trait     => "trait",
            Self::True      => "true",
            Self::Type      => "type",
            Self::Use       => "use",
            Self::While     => "while",
        }
    }
}

impl Display
for Keyword
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr
for Keyword
{
    type Err = ();

    fn from_str(
        s: &str
    ) -> Result<Self, Self::Err> {
        for token in Self::iter() {
            if s.starts_with(token.as_str()) {
                return Ok(token)
            }
        }

        Err(())
    }
}
