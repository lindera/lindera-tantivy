use tantivy::tokenizer::{Token, TokenStream};

use crate::tokenizer::Token as LToken;

pub struct LinderaTokenStream<'a> {
    pub result: Vec<LToken<'a>>,
    pub index: usize,
    pub offset_from: usize,
    pub token: Token,
}

impl<'a> TokenStream for LinderaTokenStream<'a> {
    fn advance(&mut self) -> bool {
        if self.index < self.result.len() {
            let token = self.result.get(self.index).unwrap();

            self.token = Token {
                offset_from: self.offset_from,
                offset_to: self.offset_from + token.text.len(),
                position: self.index,
                text: token.text.to_string(),
                position_length: self.result.len(),
            };

            self.offset_from += token.text.len();
            self.index += 1;

            true
        } else {
            false
        }
    }

    fn token(&self) -> &Token {
        &self.token
    }

    fn token_mut(&mut self) -> &mut Token {
        &mut self.token
    }
}
