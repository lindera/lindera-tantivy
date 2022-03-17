use tantivy::tokenizer::{BoxTokenStream, Tokenizer};

use lindera::tokenizer::{Tokenizer as LTokenizer, TokenizerConfig};
use lindera::LinderaResult;

use crate::stream::LinderaTokenStream;

/// Tokenize text with the specified mode and dictionary.
pub struct LinderaTokenizer {
    pub tokenizer: LTokenizer,
}

impl Clone for LinderaTokenizer {
    fn clone(&self) -> Self {
        Self {
            tokenizer: self.tokenizer.clone(),
        }
    }
}

impl LinderaTokenizer {
    pub fn new() -> LinderaResult<LinderaTokenizer> {
        Ok(LinderaTokenizer {
            tokenizer: LTokenizer::new()?,
        })
    }

    pub fn with_config(config: TokenizerConfig) -> LinderaResult<LinderaTokenizer> {
        Ok(LinderaTokenizer {
            tokenizer: LTokenizer::with_config(config)?,
        })
    }
}

impl Tokenizer for LinderaTokenizer {
    fn token_stream<'a>(&self, text: &'a str) -> BoxTokenStream<'a> {
        let result = match self.tokenizer.tokenize(text) {
            Ok(result) => result,
            Err(_err) => Vec::new(),
        };

        BoxTokenStream::from(LinderaTokenStream {
            result,
            token: Default::default(),
            index: 0,
            offset_from: 0,
        })
    }
}

#[cfg(test)]
#[cfg(feature = "ipadic")]
mod tests {
    use tantivy::tokenizer::{BoxTokenStream, Token, Tokenizer};

    use lindera::mode::{Mode, Penalty};
    use lindera::tokenizer::{DictionaryType, TokenizerConfig, UserDictionaryType};

    use crate::tokenizer::LinderaTokenizer;

    fn test_helper(mut tokenizer: BoxTokenStream) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokenizer.process(&mut |token: &Token| tokens.push(token.clone()));
        tokens
    }

    #[test]
    fn test_tokenizer_equal() {
        let config = TokenizerConfig {
            dict_type: DictionaryType::Ipadic,
            dict_path: None,
            user_dict_path: None,
            user_dict_type: UserDictionaryType::Csv,
            mode: Mode::Decompose(Penalty::default()),
        };

        let tokens = test_helper(
            LinderaTokenizer::with_config(config)
                .unwrap()
                .token_stream("すもももももももものうち"),
        );
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
