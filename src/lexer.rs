use crate::{
    error::TokenParseError,
    token::Token,
};

pub struct Lexer<'s>
{
    input: &'s str,
    index: usize,
}

impl<'s> Lexer<'s>
{
    pub fn new(
        input: &'s str
    ) -> Self {
        Self{
            input,
            index: 0,
        }
    }

    pub fn next_token(
        &mut self
    ) -> Result<Token, TokenParseError> {
        if let Some(idx) = self.input[self.index ..].chars()
            .enumerate()
            .find_map(|(i, c)| match c.is_whitespace() {
                true => None,
                false => Some(i),
            })
        {
            self.index += idx;
        }

        let input = &self.input[self.index ..];

        match input.is_empty() {
            false => {
                let token = Token::try_from(input)?;
                self.index += token.len();
                
                Ok(token)
            },
            true => Ok(Token::end_of_input(input)),
        }
    }
}
