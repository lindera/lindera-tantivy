use std::path::Path;

use tantivy::Result;
use tantivy::TantivyError;
use tantivy_tokenizer_api::{Token, Tokenizer};

use lindera::character_filter::BoxCharacterFilter;
use lindera::token_filter::BoxTokenFilter;
use lindera::tokenizer::{Tokenizer as LTokenizer, TokenizerBuilder};

use crate::stream::LinderaTokenStream;

#[derive(Clone)]
pub struct LinderaTokenizer {
    tokenizer: LTokenizer,
    token: Token,
}

impl LinderaTokenizer {
    /// Create a new `LinderaTokenizer`.
    /// This function will create a new `LinderaTokenizer` with settings from the YAML file specified in the `LINDERA_CONFIG_PATH` environment variable.
    pub fn new() -> Result<LinderaTokenizer> {
        let builder =
            TokenizerBuilder::new().map_err(|e| TantivyError::InvalidArgument(format!("{e:?}")))?;
        let tokenizer = builder
            .build()
            .map_err(|e| TantivyError::InvalidArgument(format!("{e:?}")))?;
        Ok(LinderaTokenizer {
            tokenizer,
            token: Default::default(),
        })
    }

    /// Create a new `LinderaTokenizer`.
    /// This function will create a new `LinderaTokenizer` with settings from the YAML file.
    pub fn from_file(file_path: &Path) -> Result<LinderaTokenizer> {
        let builder = TokenizerBuilder::from_file(file_path)
            .map_err(|e| TantivyError::InvalidArgument(format!("{e:?}")))?;
        let tokenizer = builder
            .build()
            .map_err(|e| TantivyError::InvalidArgument(format!("{e:?}")))?;
        Ok(LinderaTokenizer {
            tokenizer,
            token: Default::default(),
        })
    }

    /// Create a new `LinderaTokenizer`.
    /// This function will create a new `LinderaTokenizer` with the specified `lindera::segmenter::Segmenter`.
    pub fn from_segmenter(segmenter: lindera::segmenter::Segmenter) -> LinderaTokenizer {
        LinderaTokenizer {
            tokenizer: LTokenizer::new(segmenter),
            token: Default::default(),
        }
    }

    /// Append a character filter to the tokenizer.
    pub fn append_character_filter(&mut self, character_filter: BoxCharacterFilter) -> &mut Self {
        self.tokenizer.append_character_filter(character_filter);

        self
    }

    /// Append a token filter to the tokenizer.
    pub fn append_token_filter(&mut self, token_filter: BoxTokenFilter) -> &mut Self {
        self.tokenizer.token_filters.push(token_filter);

        self
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
    feature = "embedded-ipadic",
    feature = "embedded-unidic",
    feature = "embedded-ko-dic",
    feature = "embedded-cc-cedict"
))]
mod tests {
    use lindera::segmenter::Segmenter;
    use tantivy_tokenizer_api::{Token, TokenStream, Tokenizer};

    use lindera::dictionary::load_dictionary;
    use lindera::mode::Mode;

    use super::LinderaTokenizer;

    fn token_stream_helper(text: &str, dictionary_uri: &str) -> Vec<Token> {
        let mode = Mode::Normal;
        let dictionary = load_dictionary(dictionary_uri).unwrap();
        let user_dictionary = None;
        let segmenter = Segmenter::new(mode, dictionary, user_dictionary);
        let mut tokenizer = LinderaTokenizer::from_segmenter(segmenter);

        let mut token_stream = tokenizer.token_stream(text);
        let mut tokens: Vec<Token> = vec![];
        let mut add_token = |token: &Token| {
            tokens.push(token.clone());
        };
        token_stream.process(&mut add_token);

        tokens
    }

    #[cfg(feature = "embedded-ipadic")]
    fn token_stream_helper_ipadic(text: &str) -> Vec<Token> {
        token_stream_helper(text, "embedded://ipadic")
    }

    #[cfg(feature = "embedded-unidic")]
    fn token_stream_helper_unidic(text: &str) -> Vec<Token> {
        token_stream_helper(text, "embedded://unidic")
    }

    #[cfg(feature = "embedded-ko-dic")]
    fn token_stream_helper_kodic(text: &str) -> Vec<Token> {
        token_stream_helper(text, "embedded://ko-dic")
    }

    #[cfg(feature = "embedded-cc-cedict")]
    fn token_stream_helper_cccedict(text: &str) -> Vec<Token> {
        token_stream_helper(text, "embedded://cc-cedict")
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
    #[cfg(feature = "embedded-ipadic")]
    fn test_tokenize_ipadic() {
        let tokens = token_stream_helper_ipadic("羽田空港限定トートバッグ");
        assert_eq!(tokens.len(), 3);
        assert_token(&tokens[0], 0, "羽田空港", 0, 12);
        assert_token(&tokens[1], 1, "限定", 12, 18);
        assert_token(&tokens[2], 2, "トートバッグ", 18, 36);
    }

    #[test]
    #[cfg(feature = "embedded-unidic")]
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
    #[cfg(feature = "embedded-ko-dic")]
    fn test_tokenize_kodic() {
        let tokens = token_stream_helper_kodic("하네다공항한정토트백");
        assert_eq!(tokens.len(), 4);
        assert_token(&tokens[0], 0, "하네다", 0, 9);
        assert_token(&tokens[1], 1, "공항", 9, 15);
        assert_token(&tokens[2], 2, "한정", 15, 21);
        assert_token(&tokens[3], 3, "토트백", 21, 30);
    }

    #[test]
    #[cfg(feature = "embedded-cc-cedict")]
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
