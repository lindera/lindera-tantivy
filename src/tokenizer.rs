use crate::stream::LinderaTokenStream;
use lindera::tokenizer::Tokenizer as LTokenizer;
use tantivy::tokenizer::{BoxTokenStream, Tokenizer};

/// Tokenize text with the specified mode and dictionary.
///
/// Example: `すもももももももものうち` would be tokenized as (mode: "normal", dict: "")
///
/// | Term     | すもも | も     | もも   | も    | もも   | の    | うち   |
/// |----------|--------|--------|--------|--------|--------|--------|--------|
/// | Position | 0      | 1      | 2      | 3      | 4      | 5      | 6      |
/// | Offsets  | 0,9    | 9,12   | 12,18  | 18,21  | 21,27  | 27,30  | 30,36  |
///
/// # Example
///
/// ```rust
/// use tantivy_lindera::tokenizer::*;
/// use tantivy::tokenizer::Tokenizer;
///
/// let tokenizer = LinderaTokenizer::new("normal", "");
/// let mut stream = tokenizer.token_stream("すもももももももものうち");
/// {
///     let token = stream.next().unwrap();
///     assert_eq!(token.text, "すもも");
///     assert_eq!(token.offset_from, 0);
///     assert_eq!(token.offset_to, 9);
/// }
/// {
///   let token = stream.next().unwrap();
///     assert_eq!(token.text, "も");
///     assert_eq!(token.offset_from, 9);
///     assert_eq!(token.offset_to, 12);
/// }
/// {
///   let token = stream.next().unwrap();
///     assert_eq!(token.text, "もも");
///     assert_eq!(token.offset_from, 12);
///     assert_eq!(token.offset_to, 18);
/// }
/// {
///   let token = stream.next().unwrap();
///     assert_eq!(token.text, "も");
///     assert_eq!(token.offset_from, 18);
///     assert_eq!(token.offset_to, 21);
/// }
/// {
///   let token = stream.next().unwrap();
///     assert_eq!(token.text, "もも");
///     assert_eq!(token.offset_from, 21);
///     assert_eq!(token.offset_to, 27);
/// }
/// {
///   let token = stream.next().unwrap();
///     assert_eq!(token.text, "の");
///     assert_eq!(token.offset_from, 27);
///     assert_eq!(token.offset_to, 30);
/// }
/// {
///   let token = stream.next().unwrap();
///   assert_eq!(token.text, "うち");
///   assert_eq!(token.offset_from, 30);
///   assert_eq!(token.offset_to, 36);
/// }
/// assert!(stream.next().is_none());
/// ```
#[derive(Clone)]
pub struct LinderaTokenizer {
    pub tokenizer: LTokenizer,
}

impl LinderaTokenizer {
    pub fn new(mode: &str, dict: &str) -> LinderaTokenizer {
        LinderaTokenizer {
            tokenizer: LTokenizer::new(mode, dict),
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
        let tokens = test_helper(
            LinderaTokenizer::new("normal", "").token_stream("すもももももももものうち"),
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
