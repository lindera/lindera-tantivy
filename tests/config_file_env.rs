//! Tests that `LinderaTokenizer::new()` reads the configuration file named by the
//! `LINDERA_CONFIG_FILE` environment variable.
//!
//! This file holds a single test because the test changes process-wide environment
//! variables. Cargo builds each file under `tests/` into its own executable, so no other
//! test reads the environment while this one changes it.

#![cfg(feature = "embed-ipadic")]

use std::path::PathBuf;

use lindera_tantivy::tokenizer::LinderaTokenizer;
use tantivy_tokenizer_api::{TokenStream, Tokenizer};

/// Collects the texts of the tokens that `tokenizer` produces for `text`.
fn token_texts(tokenizer: &mut LinderaTokenizer, text: &str) -> Vec<String> {
    let mut token_stream = tokenizer.token_stream(text);
    let mut texts = Vec::new();
    while token_stream.advance() {
        texts.push(token_stream.token().text.clone());
    }
    texts
}

#[test]
fn test_new_reads_lindera_config_file() {
    // SAFETY: this is the only test in this executable, so no other thread reads or
    // writes the environment at the same time.
    unsafe {
        std::env::remove_var("LINDERA_CONFIG_FILE");
        std::env::remove_var("LINDERA_CONFIG_PATH");
    }
    assert!(
        LinderaTokenizer::new().is_err(),
        "new() should fail when no configuration file is named"
    );

    let config_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("lindera.yml");
    // SAFETY: as above.
    unsafe {
        std::env::set_var("LINDERA_CONFIG_FILE", &config_file);
    }
    let mut tokenizer = LinderaTokenizer::new()
        .expect("new() should build the tokenizer configured by LINDERA_CONFIG_FILE");

    // The mapping filter of examples/lindera.yml turns "リンデラ" into "Lindera", and its
    // lowercase filter turns that into "lindera", so the output shows that the
    // configuration file was applied.
    assert_eq!(token_texts(&mut tokenizer, "リンデラ"), ["lindera"]);
}
