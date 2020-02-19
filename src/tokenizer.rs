use lindera::tokenizer::{Token as LToken, Tokenizer as LTokenizer};
use tantivy::tokenizer::{BoxTokenStream, Token, TokenStream, Tokenizer};

pub struct LinderaTokenStream<'a> {
    result: Vec<LToken<'a>>,
    token: Token,
}

impl<'a> TokenStream for LinderaTokenStream<'a> {
    fn advance(&mut self) -> bool {
        false
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
    fn new() -> LinderaTokenizer {
        LinderaTokenizer {
            tokenizer: LTokenizer::default_normal(),
        }
    }
}

impl Tokenizer for LinderaTokenizer {
    fn token_stream<'a>(&self, text: &'a str) -> BoxTokenStream<'a> {
        let mut tokenizer = &self.tokenizer;
        let result = tokenizer.tokenize(text);

        BoxTokenStream::from(LinderaTokenStream {
            result,
            token: Default::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::LinderaTokenizer;
    use tantivy::tokenizer::{BoxTokenStream, Token, Tokenizer};
    //    use tantivy::tokenizer::tests::assert_token;

    fn test_helper(mut tokenizer: BoxTokenStream) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokenizer.process(&mut |token: &Token| tokens.push(token.clone()));
        tokens
    }

    #[test]
    fn test_tokenizer_equal() {
        let tokens = test_helper(LinderaTokenizer::new().token_stream("hello"));
        assert_eq!(tokens.len(), 3);
        //        assert_token(&tokens[0], 0, "hel", 0, 3);
        //        assert_token(&tokens[1], 0, "ell", 1, 4);
        //        assert_token(&tokens[2], 0, "llo", 2, 5);
    }
}
