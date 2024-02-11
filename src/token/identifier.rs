use std::{fmt::Display, ops::Deref, str::FromStr};

use crate::error::ParseIdentError as IdentError;

#[derive(Clone, Debug)]
#[derive(PartialEq, Hash)]
#[derive(PartialOrd)]
pub struct Identifier(String);

impl FromStr
for Identifier
{
    type Err = IdentError;

    fn from_str(
        s: &str
    ) -> Result<Self, Self::Err> {
        for (i, c) in s.char_indices()
        {
            if i == 0 {
                match c {
                    '_' => continue,
                    _ if c.is_ascii_alphabetic() => continue,
                    _ => Err(IdentError::InvalidStartCharacter)?,
                }
            }
            else {
                match c {
                    '_' => continue,
                    _ if c.is_ascii_alphanumeric() => continue,
                    _ if c.is_alphanumeric() => Err(IdentError::InvalidOtherCharacter)?,
                    _ => {
                        let ident = &s[0..i];
                        return Ok(Self(ident.to_string()))
                    }
                }
            }
        }

        Err(IdentError::Empty)
    }
}

impl Display
for Identifier
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Deref
for Identifier
{
    type Target = String;

    fn deref(
        &self
    ) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str>
for Identifier
{
    fn as_ref(
        &self
    ) -> &str {
        self.0.as_ref()
    }
}
