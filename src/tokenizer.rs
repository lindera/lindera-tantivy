use lindera_core::{
    dictionary::{Dictionary, UserDictionary},
    mode::Mode,
};
use lindera_tokenizer::tokenizer::Tokenizer as LTokenizer;
use tantivy_tokenizer_api::{Token, Tokenizer};

use crate::stream::LinderaTokenStream;

#[derive(Clone)]
pub struct LinderaTokenizer {
    tokenizer: LTokenizer,
    token: Token,
}

impl LinderaTokenizer {
    pub fn new(
        dictionary: Dictionary,
        user_dictionary: Option<UserDictionary>,
        mode: Mode,
    ) -> LinderaTokenizer {
        LinderaTokenizer {
            tokenizer: LTokenizer::new(dictionary, user_dictionary, mode),
            token: Default::default(),
        }
    }
}

impl Tokenizer for LinderaTokenizer {
    type TokenStream<'a> = LinderaTokenStream<'a>;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> LinderaTokenStream<'a> {
        self.token.reset();
        LinderaTokenStream {
            tokens: self.tokenizer.tokenize(text).unwrap(),
            token: &mut self.token,
        }
    }
}

#[cfg(test)]
#[cfg(any(
    feature = "ipadic",
    feature = "unidic",
    feature = "ko-dic",
    feature = "cc-cedict"
))]
mod tests {
    use tantivy_tokenizer_api::{Token, TokenStream, Tokenizer};

    use lindera_core::mode::Mode;
    use lindera_dictionary::{DictionaryLoader, DictionaryConfig, DictionaryKind};

    use super::LinderaTokenizer;

    fn token_stream_helper(text: &str, dictionary_kind: DictionaryKind) -> Vec<Token> {
        let dictionary_config = DictionaryConfig {
            kind: Some(dictionary_kind),
            path: None,
        };
        let dictionary = DictionaryLoader::load_dictionary_from_config(dictionary_config).unwrap();
        let mut tokenizer = LinderaTokenizer::new(dictionary, None, Mode::Normal);

        let mut token_stream = tokenizer.token_stream(text);
        let mut tokens: Vec<Token> = vec![];
        let mut add_token = |token: &Token| {
            tokens.push(token.clone());
        };
        token_stream.process(&mut add_token);

        tokens
    }

    #[cfg(feature = "ipadic")]
    fn token_stream_helper_ipadic(text: &str) -> Vec<Token> {
        token_stream_helper(text, DictionaryKind::IPADIC)
    }

    #[cfg(feature = "unidic")]
    fn token_stream_helper_unidic(text: &str) -> Vec<Token> {
        token_stream_helper(text, DictionaryKind::UniDic)
    }

    #[cfg(feature = "ko-dic")]
    fn token_stream_helper_kodic(text: &str) -> Vec<Token> {
        token_stream_helper(text, DictionaryKind::KoDic)
    }

    #[cfg(feature = "cc-cedict")]
    fn token_stream_helper_cccedict(text: &str) -> Vec<Token> {
        token_stream_helper(text, DictionaryKind::CcCedict)
    }

    /// This is a function that can be used in tests and doc tests
    /// to assert a token's correctness.
    pub fn assert_token(token: &Token, position: usize, text: &str, from: usize, to: usize) {
        assert_eq!(
            token.position, position,
            "expected position {position} but {token:?}"
        );
        assert_eq!(token.text, text, "expected text {text} but {token:?}");
        assert_eq!(
            token.offset_from, from,
            "expected offset_from {from} but {token:?}"
        );
        assert_eq!(token.offset_to, to, "expected offset_to {to} but {token:?}");
    }

    #[test]
    #[cfg(feature = "ipadic")]
    fn test_tokenize_ipadic() {
        let tokens = token_stream_helper_ipadic("羽田空港限定トートバッグ");
        assert_eq!(tokens.len(), 3);
        assert_token(&tokens[0], 0, "羽田空港", 0, 12);
        assert_token(&tokens[1], 1, "限定", 12, 18);
        assert_token(&tokens[2], 2, "トートバッグ", 18, 36);
    }

    #[test]
    #[cfg(feature = "unidic")]
    fn test_tokenize_unidic() {
        let tokens = token_stream_helper_unidic("羽田空港限定トートバッグ");
        assert_eq!(tokens.len(), 5);
        assert_token(&tokens[0], 0, "羽田", 0, 6);
        assert_token(&tokens[1], 1, "空港", 6, 12);
        assert_token(&tokens[2], 2, "限定", 12, 18);
        assert_token(&tokens[3], 3, "トート", 18, 27);
        assert_token(&tokens[4], 4, "バッグ", 27, 36);
    }

    #[test]
    #[cfg(feature = "ko-dic")]
    fn test_tokenize_kodic() {
        let tokens = token_stream_helper_kodic("하네다공항한정토트백");
        assert_eq!(tokens.len(), 4);
        assert_token(&tokens[0], 0, "하네다", 0, 9);
        assert_token(&tokens[1], 1, "공항", 9, 15);
        assert_token(&tokens[2], 2, "한정", 15, 21);
        assert_token(&tokens[3], 3, "토트백", 21, 30);
    }

    #[test]
    #[cfg(feature = "cc-cedict")]
    fn test_tokenize_cccedict() {
        let tokens = token_stream_helper_cccedict("羽田机场限量版手提包");
        assert_eq!(tokens.len(), 6);
        assert_token(&tokens[0], 0, "羽田", 0, 6);
        assert_token(&tokens[1], 1, "机场", 6, 12);
        assert_token(&tokens[2], 2, "限", 12, 15);
        assert_token(&tokens[3], 3, "量", 15, 18);
        assert_token(&tokens[4], 4, "版", 18, 21);
        assert_token(&tokens[5], 5, "手提包", 21, 30);
    }
}
