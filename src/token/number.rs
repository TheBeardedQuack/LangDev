use std::{
    fmt::Display,
    num::IntErrorKind as NumberError,
    str::FromStr,
};

use crate::error::TokenParseError;

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(PartialOrd, Ord)]
pub struct Number{
    value: i128,
    str_len: usize,
}

impl Number
{
    #[allow(clippy::len_without_is_empty)]
    pub fn len(
        &self
    ) -> usize {
        self.str_len
    }

    pub fn val(
        &self
    ) -> i128 {
        self.value
    }
}

impl FromStr
for Number
{
    type Err = TokenParseError;

    fn from_str(
        s: &str
    ) -> Result<Self, Self::Err> {
        for (i, c) in s.char_indices()
        {
            match c {
                '0'..='9' => continue,
                _ if c.is_ascii_alphanumeric() => Err(Self::Err::ParseIntError(NumberError::InvalidDigit))?,
                _ => {
                    let substr = &s[0..i];
                    if substr.is_empty() {
                        break;      // Drop-out to return default TokenEmpty error
                    }

                    return Ok(Self{
                        value: substr.parse::<i128>()?,
                        str_len: substr.len(),
                    })
                },
            }
        }

        Err(Self::Err::ParseIntError(NumberError::Empty))
    }
}

impl Display
for Number
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
