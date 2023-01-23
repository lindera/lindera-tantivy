use std::collections::{HashSet, VecDeque};

use tantivy::tokenizer::{BoxTokenStream, Token as TToken, Tokenizer as TTokenizer};

use lindera::{
    analyzer::Analyzer,
    builder,
    character_filter::unicode_normalize::{
        UnicodeNormalizeCharacterFilter, UnicodeNormalizeCharacterFilterConfig,
        UnicodeNormalizeKind,
    },
    token_filter::{
        japanese_compound_word::{
            JapaneseCompoundWordTokenFilter, JapaneseCompoundWordTokenFilterConfig,
        },
        japanese_number::{JapaneseNumberTokenFilter, JapaneseNumberTokenFilterConfig},
    },
    tokenizer::{
        DictionaryConfig as LDictionaryConfig, Tokenizer as LTokenizer,
        TokenizerConfig as LTokenizerConfig,
    },
    BoxCharacterFilter, BoxTokenFilter, DictionaryKind as LDictionaryKind, Token as LToken,
};

// use crate::LinderaResult;
use crate::{mode::Mode, stream::LinderaTokenStream};

pub type DictionaryConfig = LDictionaryConfig;
pub type DictionaryKind = LDictionaryKind;
pub type TokenizerConfig = LTokenizerConfig;
pub type Token<'a> = LToken<'a>;

pub struct LinderaTokenizer {
    pub analyzer: Analyzer,
}

impl Clone for LinderaTokenizer {
    fn clone(&self) -> Self {
        Self {
            analyzer: self.analyzer.clone(),
        }
    }
}

impl LinderaTokenizer {
    pub fn new(
        character_filters: Vec<BoxCharacterFilter>,
        tokenizer: LTokenizer,
        token_filters: Vec<BoxTokenFilter>,
    ) -> LinderaTokenizer {
        LinderaTokenizer {
            analyzer: Analyzer::new(character_filters, tokenizer, token_filters),
        }
    }
}

impl Default for LinderaTokenizer {
    fn default() -> Self {
        // Add character filters.
        let mut character_filters: Vec<BoxCharacterFilter> = Vec::new();
        // Unicode normalize character filter
        character_filters.push(BoxCharacterFilter::from(
            UnicodeNormalizeCharacterFilter::new(UnicodeNormalizeCharacterFilterConfig::new(
                UnicodeNormalizeKind::NFKC,
            )),
        ));

        // Tokenizer with IPADIC
        let dictionary = builder::load_dictionary_from_kind(DictionaryKind::IPADIC).unwrap();
        let tokenizer = LTokenizer::new(dictionary, None, Mode::Normal);

        // Add token filters.
        let mut token_filters: Vec<BoxTokenFilter> = Vec::new();
        // Japanese compound word token filter
        token_filters.push(BoxTokenFilter::from(JapaneseCompoundWordTokenFilter::new(
            JapaneseCompoundWordTokenFilterConfig::new(
                DictionaryKind::IPADIC,
                HashSet::from(["名詞,数".to_string()]),
                Some("名詞,数".to_string()),
            ),
        )));
        // Japanese number token filter
        token_filters.push(BoxTokenFilter::from(JapaneseNumberTokenFilter::new(
            JapaneseNumberTokenFilterConfig::new(Some(HashSet::from(["名詞,数".to_string()]))),
        )));

        Self::new(character_filters, tokenizer, token_filters)
    }
}

impl TTokenizer for LinderaTokenizer {
    fn token_stream<'a>(&self, text: &'a str) -> BoxTokenStream<'a> {
        let tokens = match self.analyzer.analyze(&mut text.to_string()) {
            Ok(lindera_tokens) => lindera_tokens
                .iter()
                .map(|lindera_token| TToken {
                    offset_from: lindera_token.byte_start,
                    offset_to: lindera_token.byte_end,
                    position: lindera_token.position,
                    text: lindera_token.get_text().to_string(),
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
        let tokens =
            test_helper(LinderaTokenizer::default().token_stream("Ｌｉｎｄｅｒａは形態素解析ｴﾝｼﾞﾝです。"));
        assert_eq!(tokens.len(), 7);
        {
            let token = &tokens[0];
            assert_eq!(token.text, "Lindera");
            assert_eq!(token.offset_from, 0);
            assert_eq!(token.offset_to, 21);
            assert_eq!(token.position, 0);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[1];
            assert_eq!(token.text, "は");
            assert_eq!(token.offset_from, 21);
            assert_eq!(token.offset_to, 24);
            assert_eq!(token.position, 1);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[2];
            assert_eq!(token.text, "形態素");
            assert_eq!(token.offset_from, 24);
            assert_eq!(token.offset_to, 33);
            assert_eq!(token.position, 2);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[3];
            assert_eq!(token.text, "解析");
            assert_eq!(token.offset_from, 33);
            assert_eq!(token.offset_to, 39);
            assert_eq!(token.position, 3);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[4];
            assert_eq!(token.text, "エンジン");
            assert_eq!(token.offset_from, 39);
            assert_eq!(token.offset_to, 54);
            assert_eq!(token.position, 4);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[5];
            assert_eq!(token.text, "です");
            assert_eq!(token.offset_from, 54);
            assert_eq!(token.offset_to, 60);
            assert_eq!(token.position, 5);
            assert_eq!(token.position_length, 1);
        }
        {
            let token = &tokens[6];
            assert_eq!(token.text, "。");
            assert_eq!(token.offset_from,  60);
            assert_eq!(token.offset_to, 63);
            assert_eq!(token.position, 6);
            assert_eq!(token.position_length, 1);
        }
    }
}
