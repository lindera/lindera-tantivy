# Lindera tokenizer for Tantivy

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Join the chat at https://gitter.im/lindera-morphology/lindera](https://badges.gitter.im/lindera-morphology/lindera.svg)](https://gitter.im/lindera-morphology/lindera?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

A Tokenizer for [Tantivy](https://github.com/tantivy-search/tantivy), based on [Lindera](https://github.com/lindera-morphology/lindera).

## Build

The following products are required to build:

- Rust >= 1.39.0
- make >= 3.81

```text
% make build
```

## Usage

### Basic example

```rust
use tantivy_lindera::tokenizer::*;
use tantivy::tokenizer::Tokenizer;

fn main() -> std::io::Result<()> {
    let tokenizer = LinderaTokenizer::new("normal", "");
    let mut stream = tokenizer.token_stream("すもももももももものうち");
    {
        let token = stream.next().unwrap();
        assert_eq!(token.text, "すもも");
        assert_eq!(token.offset_from, 0);
        assert_eq!(token.offset_to, 9);
    }
    {
        let token = stream.next().unwrap();
        assert_eq!(token.text, "も");
        assert_eq!(token.offset_from, 9);
        assert_eq!(token.offset_to, 12);
    }
    {
        let token = stream.next().unwrap();
        assert_eq!(token.text, "もも");
        assert_eq!(token.offset_from, 12);
       assert_eq!(token.offset_to, 18);
    }
    {
        let token = stream.next().unwrap();
        assert_eq!(token.text, "も");
        assert_eq!(token.offset_from, 18);
        assert_eq!(token.offset_to, 21);
    }
    {
        let token = stream.next().unwrap();
        assert_eq!(token.text, "もも");
        assert_eq!(token.offset_from, 21);  
        assert_eq!(token.offset_to, 27);
    }
    {
        let token = stream.next().unwrap();
        assert_eq!(token.text, "の");
        assert_eq!(token.offset_from, 27);  
        assert_eq!(token.offset_to, 30);
    }
    {
        let token = stream.next().unwrap();
        assert_eq!(token.text, "うち");
        assert_eq!(token.offset_from, 30);
        assert_eq!(token.offset_to, 36);
    }
    assert!(stream.next().is_none());

    Ok(())
}
```

## API reference

The API reference is available. Please see following URL:
- <a href="https://docs.rs/tantivy-lindera" target="_blank">tantivy-lindera</a>
