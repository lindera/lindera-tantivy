use lindera::tokenizer::{Token as LToken, Tokenizer as LTokenizer};
use tantivy::tokenizer::{BoxTokenStream, Token, TokenStream, Tokenizer};

pub struct LinderaTokenStream<'a> {
    result: Vec<LToken<'a>>,
    index: usize,
    offset_from: usize,
    token: Token,
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

#[derive(Clone)]
pub struct LinderaTokenizer {
    pub tokenizer: LTokenizer,
}

impl LinderaTokenizer {
    pub fn new() -> LinderaTokenizer {
        LinderaTokenizer {
            tokenizer: LTokenizer::default_normal(),
        }
    }
}

impl Tokenizer for LinderaTokenizer {
    fn token_stream<'a>(&self, text: &'a str) -> BoxTokenStream<'a> {
        let mut tokenizer = self.tokenizer.clone();
        let result = tokenizer.tokenize(text);

        BoxTokenStream::from(LinderaTokenStream {
            result,
            token: Default::default(),
            index: 0,
            offset_from: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::LinderaTokenizer;
    use tantivy::tokenizer::{BoxTokenStream, Token, Tokenizer};

    fn test_helper(mut tokenizer: BoxTokenStream) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokenizer.process(&mut |token: &Token| tokens.push(token.clone()));
        tokens
    }

    #[test]
    fn test_tokenizer_equal() {
        let tokens = test_helper(LinderaTokenizer::new().token_stream("すもももももももものうち"));
        assert_eq!(tokens.len(), 7);
        {
            let token = &tokens[0];
            assert_eq!(token.text, "すもも");
            assert_eq!(token.offset_from, 0);
            assert_eq!(token.offset_to, 9);
            assert_eq!(token.position, 0);
        }
        {
            let token = &tokens[1];
            assert_eq!(token.text, "も");
            assert_eq!(token.offset_from, 9);
            assert_eq!(token.offset_to, 12);
            assert_eq!(token.position, 1);
        }
        {
            let token = &tokens[2];
            assert_eq!(token.text, "もも");
            assert_eq!(token.offset_from, 12);
            assert_eq!(token.offset_to, 18);
            assert_eq!(token.position, 2);
        }
        {
            let token = &tokens[3];
            assert_eq!(token.text, "も");
            assert_eq!(token.offset_from, 18);
            assert_eq!(token.offset_to, 21);
            assert_eq!(token.position, 3);
        }
        {
            let token = &tokens[4];
            assert_eq!(token.text, "もも");
            assert_eq!(token.offset_from, 21);
            assert_eq!(token.offset_to, 27);
            assert_eq!(token.position, 4);
        }
        {
            let token = &tokens[5];
            assert_eq!(token.text, "の");
            assert_eq!(token.offset_from, 27);
            assert_eq!(token.offset_to, 30);
            assert_eq!(token.position, 5);
        }
        {
            let token = &tokens[6];
            assert_eq!(token.text, "うち");
            assert_eq!(token.offset_from, 30);
            assert_eq!(token.offset_to, 36);
            assert_eq!(token.position, 6);
        }
    }
}
