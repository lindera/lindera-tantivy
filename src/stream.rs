use std::collections::VecDeque;

use tantivy::tokenizer::{Token, TokenStream};

pub struct LinderaTokenStream {
    tokens: VecDeque<Token>,
    token: Token,
}

impl LinderaTokenStream {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self {
            tokens,
            token: Default::default(),
        }
    }
}

impl TokenStream for LinderaTokenStream {
    fn advance(&mut self) -> bool {
        match self.tokens.pop_front() {
            Some(token) => {
                self.token = token;
                true
            }
            None => false,
        }
    }

    fn token(&self) -> &Token {
        &self.token
    }

    fn token_mut(&mut self) -> &mut Token {
        &mut self.token
    }
}
