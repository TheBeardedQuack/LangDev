use std::{fmt::Display, str::FromStr};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(EnumIter)]
pub enum Symbol
{
    // End of input (file or str)
    EndOfInput,

    // Compare
    Equal,          //      ==
    NotEqual,       //      !=
    LThanEq,        //      <=
    RThanEq,        //      >=
    SingleArrow,    //      ->
    DoubleArrow,    //      =>
    LogicAnd,       //      &&
    LogicOr,        //      ||

    // Assign Ops
    AddAssign,      //      +=
    SubAssign,      //      -=
    MulAssign,      //      *=
    FSlashAssign,   //      /=
    BSlashAssign,   //      \=
    XorAssign,      //      ^=

    // Operators (need to come before single char versions)
    LShift,         //      <<
    RShift,         //      >>

    // Operators
    Assign,         //      =
    Plus,           //      +
    Minus,          //      -
    Asterisk,       //      *
    FSlash,         //      /
    BSlash,         //      \
    Xor,            //      ^
    Exclaim,        //      !
    Question,       //      ?
    And,            //      &
    Pipe,           //      |
    Tilde,          //      ~

    // Strings
    SingleQuote,    //      '
    DoubleQuote,    //      "

    // Brace pairs
    OpenBrace,      //      (
    CloseBrace,     //      )
    OpenCurly,      //      {
    CloseCurly,     //      }
    OpenSquare,     //      [
    CloseSquare,    //      ]
    OpenAngle,      //      <       Start-generic and less-than operator
    CloseAngle,     //      >       End-generic and greater-than operator

    // Other
    Scope,      //      ::      Must come before Colon  ":"
    Range,      //      ..      Must come before Dot    "."
    Colon,      //      :
    SemiColon,  //      ;
    Dot,        //      .
    Comma,      //      ,
    Hash,       //      #
}

impl Symbol
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
            // End of input (file or str)
            Self::EndOfInput    => "\0",
        
            // Operators (need to come before single char strings)
            Self::LShift    => "<<",
            Self::RShift    => ">>",
        
            // Compare
            Self::Equal         => "==",
            Self::NotEqual      => "!=",
            Self::LThanEq       => "<=",
            Self::RThanEq       => ">=",
            Self::SingleArrow   => "->",
            Self::DoubleArrow   => "=>",
            Self::LogicAnd      => "&&",
            Self::LogicOr       => "||",
        
            // Assign Ops
            Self::AddAssign     => "+=",
            Self::SubAssign     => "-=",
            Self::MulAssign     => "*=",
            Self::FSlashAssign  => "/=",
            Self::BSlashAssign  => "\\=",
            Self::XorAssign     => "^=",

            // Operators
            Self::Assign    => "=",
            Self::Plus      => "+",
            Self::Minus     => "-",
            Self::Asterisk  => "*",
            Self::FSlash    => "/",
            Self::BSlash    => "\\",
            Self::Xor       => "^",
            Self::Exclaim   => "!",
            Self::Question  => "?",
            Self::And       => "&",
            Self::Pipe      => "|",
            Self::Tilde     => "~",
        
            // Strings
            Self::SingleQuote   => "'",
            Self::DoubleQuote   => "\"",
        
            // Brace pairs
            Self::OpenBrace     => "(",
            Self::CloseBrace    => ")",
            Self::OpenCurly     => "{",
            Self::CloseCurly    => "}",
            Self::OpenSquare    => "[",
            Self::CloseSquare   => "]",
            Self::OpenAngle     => "<",
            Self::CloseAngle    => ">",
        
            // Other
            Self::Scope     => "::",
            Self::Range     => "..",
            Self::Colon     => ":",
            Self::SemiColon => ";",
            Self::Dot       => ".",
            Self::Comma     => ",",
            Self::Hash      => "#",
        }
    }
}

impl Display
for Symbol
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr
for Symbol
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
