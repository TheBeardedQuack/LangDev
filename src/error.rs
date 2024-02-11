use std::num::{IntErrorKind, ParseIntError};

#[derive(Clone, Debug)]
#[derive(PartialEq, Eq)]
#[derive(thiserror::Error)]
pub enum TokenParseError
{
    #[error("found unexpected leading whitespace while parsing token")]
    LeadingWhitespace,  // This indicates an error in the lexer code

    #[error("error parsing identifier, {0}")]
    ParseIdentError(#[from] ParseIdentError),

    #[error("error parsing integer, {0:?}")]
    ParseIntError(IntErrorKind),

    #[error("native error while parsing integer, {0}")]
    NativeIntError(#[from] ParseIntError),

    #[error("returned by tryfrom if string does not match a known keyword")]
    InvalidKeyword,

    #[error("failed to parse input as a valid token")]
    InvalidToken,
}

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq)]
#[derive(thiserror::Error)]
pub enum ParseIdentError
{
    #[error("identifier name cannot be empty")]
    Empty,

    #[error("invalid first character in identifier name")]
    InvalidStartCharacter,

    #[error("invalid character found (not the first character) in identifier name")]
    InvalidOtherCharacter,
}
