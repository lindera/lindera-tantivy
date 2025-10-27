//! Lindera tokenizer implementation for Tantivy.
//!
//! This module provides the [`LinderaTokenizer`] struct, which implements Tantivy's
//! [`Tokenizer`] trait using Lindera's morphological analysis capabilities.

use std::path::Path;

use tantivy::Result;
use tantivy::TantivyError;
use tantivy_tokenizer_api::{Token, Tokenizer};

use lindera::character_filter::BoxCharacterFilter;
use lindera::token_filter::BoxTokenFilter;
use lindera::tokenizer::{Tokenizer as LTokenizer, TokenizerBuilder};

use crate::stream::LinderaTokenStream;

/// A Tantivy tokenizer that uses Lindera for morphological analysis.
///
/// `LinderaTokenizer` wraps a Lindera tokenizer and provides an implementation of
/// Tantivy's `Tokenizer` trait. It can be configured in multiple ways:
///
/// - From a Lindera `Segmenter` (programmatic configuration)
/// - From a YAML configuration file
/// - From the `LINDERA_CONFIG_PATH` environment variable
///
/// The tokenizer supports character filters and token filters to customize the
/// tokenization process.
///
/// # Examples
///
/// ## Creating from a Segmenter
///
/// ```rust,no_run
/// use lindera::dictionary::DictionaryKind;
/// use lindera::{dictionary::load_dictionary_from_kind, mode::Mode, segmenter::Segmenter};
/// use lindera_tantivy::tokenizer::LinderaTokenizer;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mode = Mode::Normal;
/// let dictionary = load_dictionary_from_kind(DictionaryKind::IPADIC)?;
/// let segmenter = Segmenter::new(mode, dictionary, None);
/// let tokenizer = LinderaTokenizer::from_segmenter(segmenter);
/// # Ok(())
/// # }
/// ```
///
/// ## Creating from a configuration file
///
/// ```rust,no_run
/// use std::path::Path;
/// use lindera_tantivy::tokenizer::LinderaTokenizer;
///
/// # fn main() -> tantivy::Result<()> {
/// let config_path = Path::new("lindera.yml");
/// let tokenizer = LinderaTokenizer::from_file(config_path)?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct LinderaTokenizer {
    tokenizer: LTokenizer,
    token: Token,
}

impl LinderaTokenizer {
    /// Creates a new `LinderaTokenizer` from the `LINDERA_CONFIG_PATH` environment variable.
    ///
    /// This method reads the path to a YAML configuration file from the `LINDERA_CONFIG_PATH`
    /// environment variable and constructs a tokenizer based on that configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The `LINDERA_CONFIG_PATH` environment variable is not set
    /// - The configuration file cannot be read or parsed
    /// - The configuration is invalid
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lindera_tantivy::tokenizer::LinderaTokenizer;
    ///
    /// # fn main() -> tantivy::Result<()> {
    /// // Assumes LINDERA_CONFIG_PATH environment variable is set
    /// let tokenizer = LinderaTokenizer::new()?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Creates a new `LinderaTokenizer` from a YAML configuration file.
    ///
    /// This method constructs a tokenizer by reading configuration from the specified
    /// YAML file path. The configuration file can specify the dictionary, mode,
    /// character filters, and token filters to use.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the YAML configuration file
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The YAML is malformed
    /// - The configuration is invalid
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::path::Path;
    /// use lindera_tantivy::tokenizer::LinderaTokenizer;
    ///
    /// # fn main() -> tantivy::Result<()> {
    /// let config_path = Path::new("lindera.yml");
    /// let tokenizer = LinderaTokenizer::from_file(config_path)?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Creates a new `LinderaTokenizer` from a Lindera `Segmenter`.
    ///
    /// This method provides direct programmatic control over the tokenizer configuration
    /// by accepting a pre-configured Lindera `Segmenter`. This is the most flexible way
    /// to create a tokenizer as it allows you to specify the exact dictionary, mode,
    /// and user dictionary to use.
    ///
    /// # Arguments
    ///
    /// * `segmenter` - A configured Lindera `Segmenter` instance
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lindera::dictionary::DictionaryKind;
    /// use lindera::{dictionary::load_dictionary_from_kind, mode::Mode, segmenter::Segmenter};
    /// use lindera_tantivy::tokenizer::LinderaTokenizer;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Create a segmenter with IPADIC dictionary in Normal mode
    /// let mode = Mode::Normal;
    /// let dictionary = load_dictionary_from_kind(DictionaryKind::IPADIC)?;
    /// let user_dictionary = None;
    /// let segmenter = Segmenter::new(mode, dictionary, user_dictionary);
    ///
    /// // Create the tokenizer
    /// let tokenizer = LinderaTokenizer::from_segmenter(segmenter);
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_segmenter(segmenter: lindera::segmenter::Segmenter) -> LinderaTokenizer {
        LinderaTokenizer {
            tokenizer: LTokenizer::new(segmenter),
            token: Default::default(),
        }
    }

    /// Appends a character filter to the tokenizer.
    ///
    /// Character filters transform the input text before tokenization. They can be used
    /// for operations like Unicode normalization, mapping characters, or removing
    /// specific characters.
    ///
    /// Multiple character filters can be chained by calling this method multiple times.
    /// The filters will be applied in the order they were added.
    ///
    /// # Arguments
    ///
    /// * `character_filter` - The character filter to append
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lindera::character_filter::japanese_iteration_mark::JapaneseIterationMarkCharacterFilter;
    /// use lindera::dictionary::DictionaryKind;
    /// use lindera::{dictionary::load_dictionary_from_kind, mode::Mode, segmenter::Segmenter};
    /// use lindera_tantivy::tokenizer::LinderaTokenizer;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mode = Mode::Normal;
    /// let dictionary = load_dictionary_from_kind(DictionaryKind::IPADIC)?;
    /// let segmenter = Segmenter::new(mode, dictionary, None);
    /// let mut tokenizer = LinderaTokenizer::from_segmenter(segmenter);
    ///
    /// // Add a character filter
    /// let char_filter = JapaneseIterationMarkCharacterFilter::new(true, true);
    /// tokenizer.append_character_filter(Box::new(char_filter));
    /// # Ok(())
    /// # }
    /// ```
    pub fn append_character_filter(&mut self, character_filter: BoxCharacterFilter) -> &mut Self {
        self.tokenizer.append_character_filter(character_filter);

        self
    }

    /// Appends a token filter to the tokenizer.
    ///
    /// Token filters transform the tokens after tokenization. They can be used for
    /// operations like lowercasing, removing stop words, or stemming.
    ///
    /// Multiple token filters can be chained by calling this method multiple times.
    /// The filters will be applied in the order they were added.
    ///
    /// # Arguments
    ///
    /// * `token_filter` - The token filter to append
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lindera::token_filter::japanese_stop_tags::JapaneseStopTagsTokenFilter;
    /// use lindera::dictionary::DictionaryKind;
    /// use lindera::{dictionary::load_dictionary_from_kind, mode::Mode, segmenter::Segmenter};
    /// use lindera_tantivy::tokenizer::LinderaTokenizer;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mode = Mode::Normal;
    /// let dictionary = load_dictionary_from_kind(DictionaryKind::IPADIC)?;
    /// let segmenter = Segmenter::new(mode, dictionary, None);
    /// let mut tokenizer = LinderaTokenizer::from_segmenter(segmenter);
    ///
    /// // Add a token filter to remove specific part-of-speech tags
    /// let tags = vec!["接続詞".to_string()];
    /// let token_filter = JapaneseStopTagsTokenFilter::new(tags);
    /// tokenizer.append_token_filter(Box::new(token_filter));
    /// # Ok(())
    /// # }
    /// ```
    pub fn append_token_filter(&mut self, token_filter: BoxTokenFilter) -> &mut Self {
        self.tokenizer.token_filters.push(token_filter);

        self
    }
}

impl Tokenizer for LinderaTokenizer {
    type TokenStream<'a> = LinderaTokenStream<'a>;

    #[inline]
    fn token_stream<'a>(&'a mut self, text: &'a str) -> LinderaTokenStream<'a> {
        self.token.reset();
        LinderaTokenStream {
            tokens: self.tokenizer.tokenize(text).unwrap(),
            token: &mut self.token,
            current_index: 0,
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
