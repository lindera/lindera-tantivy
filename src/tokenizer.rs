use std::collections::VecDeque;

use tantivy::tokenizer::{BoxTokenStream, Token, Tokenizer};

use lindera::tokenizer::Tokenizer as LTokenizer;

use crate::{
    dictionary::load_dictionary, stream::LinderaTokenStream, Dictionary, DictionaryConfig,
    DictionaryKind, Mode, UserDictionary,
};

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
    pub fn new(
        dictionary: Dictionary,
        user_dictionary: Option<UserDictionary>,
        mode: Mode,
    ) -> LinderaTokenizer {
        LinderaTokenizer {
            tokenizer: LTokenizer::new(dictionary, user_dictionary, mode),
        }
    }
}

impl Default for LinderaTokenizer {
    fn default() -> Self {
        // Dictionary.
        let dictionary = load_dictionary(DictionaryConfig {
            kind: Some(DictionaryKind::IPADIC),
            path: None,
        })
        .unwrap();

        // User dictionary.
        let user_dictionary = None;

        // Mode.
        let mode = Mode::Normal;

        Self::new(dictionary, user_dictionary, mode)
    }
}

impl Tokenizer for LinderaTokenizer {
    fn token_stream<'a>(&self, text: &'a str) -> BoxTokenStream<'a> {
        let tokens = match self.tokenizer.tokenize(text) {
            Ok(lindera_tokens) => lindera_tokens
                .iter()
                .map(|lindera_token| Token {
                    offset_from: lindera_token.byte_start,
                    offset_to: lindera_token.byte_end,
                    position: lindera_token.position,
                    text: lindera_token.text.to_string(),
                    position_length: lindera_token.position_length,
                })
                .collect::<VecDeque<_>>(),
            Err(_err) => VecDeque::new(),
        };

        BoxTokenStream::from(LinderaTokenStream::new(tokens))
    }
}

#[cfg(test)]
#[cfg(feature = "ipadic")]
mod tests {
    use tantivy::tokenizer::{BoxTokenStream, Token, Tokenizer};

    use crate::tokenizer::LinderaTokenizer;

    fn test_helper(mut tokenizer: BoxTokenStream) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokenizer.process(&mut |token: &Token| tokens.push(token.clone()));
        tokens
    }

    #[test]
    fn test_tokenizer() {
        let tokens =
            test_helper(LinderaTokenizer::default().token_stream("すもももももももものうち"));
        assert_eq!(tokens.len(), 7);
        {
            let token = &tokens[0];
            assert_eq!(token.text, "すもも");
            assert_eq!(token.offset_from, 0);
            assert_eq!(token.offset_to, 9);
            assert_eq!(token.position, 0);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[1];
            assert_eq!(token.text, "も");
            assert_eq!(token.offset_from, 9);
            assert_eq!(token.offset_to, 12);
            assert_eq!(token.position, 1);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[2];
            assert_eq!(token.text, "もも");
            assert_eq!(token.offset_from, 12);
            assert_eq!(token.offset_to, 18);
            assert_eq!(token.position, 2);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[3];
            assert_eq!(token.text, "も");
            assert_eq!(token.offset_from, 18);
            assert_eq!(token.offset_to, 21);
            assert_eq!(token.position, 3);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[4];
            assert_eq!(token.text, "もも");
            assert_eq!(token.offset_from, 21);
            assert_eq!(token.offset_to, 27);
            assert_eq!(token.position, 4);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[5];
            assert_eq!(token.text, "の");
            assert_eq!(token.offset_from, 27);
            assert_eq!(token.offset_to, 30);
            assert_eq!(token.position, 5);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[6];
            assert_eq!(token.text, "うち");
            assert_eq!(token.offset_from, 30);
            assert_eq!(token.offset_to, 36);
            assert_eq!(token.position, 6);
            assert_eq!(token.position_length, 1);
        }
    }

    #[test]
    fn test_tokenizer_lindera() {
        let tokens = test_helper(
            LinderaTokenizer::default().token_stream("Linderaは形態素解析エンジンです。"),
        );
        assert_eq!(tokens.len(), 7);
        {
            let token = &tokens[0];
            assert_eq!(token.text, "Lindera");
            assert_eq!(token.offset_from, 0);
            assert_eq!(token.offset_to, 7);
            assert_eq!(token.position, 0);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[1];
            assert_eq!(token.text, "は");
            assert_eq!(token.offset_from, 7);
            assert_eq!(token.offset_to, 10);
            assert_eq!(token.position, 1);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[2];
            assert_eq!(token.text, "形態素");
            assert_eq!(token.offset_from, 10);
            assert_eq!(token.offset_to, 19);
            assert_eq!(token.position, 2);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[3];
            assert_eq!(token.text, "解析");
            assert_eq!(token.offset_from, 19);
            assert_eq!(token.offset_to, 25);
            assert_eq!(token.position, 3);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[4];
            assert_eq!(token.text, "エンジン");
            assert_eq!(token.offset_from, 25);
            assert_eq!(token.offset_to, 37);
            assert_eq!(token.position, 4);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[5];
            assert_eq!(token.text, "です");
            assert_eq!(token.offset_from, 37);
            assert_eq!(token.offset_to, 43);
            assert_eq!(token.position, 5);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[6];
            assert_eq!(token.text, "。");
            assert_eq!(token.offset_from, 43);
            assert_eq!(token.offset_to, 46);
            assert_eq!(token.position, 6);
            assert_eq!(token.position_length, 1);
        }
    }
}
