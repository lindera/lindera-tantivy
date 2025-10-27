//! Token stream implementation for Lindera tokenizer.
//!
//! This module provides the [`LinderaTokenStream`] struct, which implements Tantivy's
//! [`TokenStream`] trait to iterate over tokens produced by Lindera's morphological analysis.

use tantivy_tokenizer_api::{Token, TokenStream};

use lindera::token::Token as LToken;

/// A token stream that iterates over tokens produced by Lindera.
///
/// `LinderaTokenStream` is created by [`LinderaTokenizer`](crate::tokenizer::LinderaTokenizer)
/// and provides access to the tokens produced by Lindera's morphological analysis.
/// It implements Tantivy's `TokenStream` trait, allowing it to be used in Tantivy's
/// indexing and search pipeline.
///
/// Each token contains information about:
/// - The surface form (text)
/// - Byte offsets in the original text
/// - Position in the token sequence
/// - Position length (for multi-token expressions)
///
/// # Note
///
/// This struct is typically not created directly by users. Instead, it's created
/// internally by `LinderaTokenizer::token_stream()`.
pub struct LinderaTokenStream<'a> {
    pub tokens: Vec<LToken<'a>>,
    pub token: &'a mut Token,
    pub current_index: usize,
}

impl<'a> TokenStream for LinderaTokenStream<'a> {
    /// Advances to the next token in the stream.
    ///
    /// This method moves the stream forward to the next token and updates the current
    /// token with its surface form, byte offsets, and position information.
    ///
    /// # Returns
    ///
    /// Returns `true` if there was a next token, `false` if the end of the stream
    /// has been reached.
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

    /// Returns a reference to the current token.
    ///
    /// # Returns
    ///
    /// An immutable reference to the current token.
    #[inline(always)]
    fn token(&self) -> &Token {
        self.token
    }

    /// Returns a mutable reference to the current token.
    ///
    /// # Returns
    ///
    /// A mutable reference to the current token, allowing for modifications
    /// such as lowercasing or stemming.
    #[inline(always)]
    fn token_mut(&mut self) -> &mut Token {
        self.token
    }
}
