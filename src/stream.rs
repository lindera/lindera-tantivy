use tantivy_tokenizer_api::{Token, TokenStream};

use lindera::token::Token as LToken;

pub struct LinderaTokenStream<'a> {
    pub tokens: Vec<LToken<'a>>,
    pub token: &'a mut Token,
    pub current_index: usize,
}

impl<'a> TokenStream for LinderaTokenStream<'a> {
    fn advance(&mut self) -> bool {
        if self.current_index >= self.tokens.len() {
            return false;
        }

        let token = &self.tokens[self.current_index];
        self.token.text = token.surface.to_string();
        self.token.offset_from = token.byte_start;
        self.token.offset_to = token.byte_end;
        self.token.position = token.position;
        self.token.position_length = token.position_length;

        self.current_index += 1;
        true
    }

    #[inline(always)]
    fn token(&self) -> &Token {
        self.token
    }

    #[inline(always)]
    fn token_mut(&mut self) -> &mut Token {
        self.token
    }
}
