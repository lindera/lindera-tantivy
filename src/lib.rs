//! Lindera tokenizer for Tantivy
//!
//! This crate provides a [Lindera](https://github.com/lindera/lindera) tokenizer implementation
//! for [Tantivy](https://github.com/tantivy-search/tantivy), a full-text search engine library.
//!
//! Lindera is a morphological analysis library that supports multiple languages including
//! Japanese, Korean, and Chinese. This integration allows you to use Lindera's tokenization
//! capabilities within Tantivy's indexing and search pipeline.
//!
//! # Features
//!
//! This crate supports multiple embedded dictionaries through feature flags:
//!
//! - `embedded-ipadic` - Japanese dictionary (IPADIC)
//! - `embedded-ipadic-neologd` - Japanese dictionary (IPADIC NEologd)
//! - `embedded-unidic` - Japanese dictionary (UniDic)
//! - `embedded-ko-dic` - Korean dictionary (ko-dic)
//! - `embedded-cc-cedict` - Chinese dictionary (CC-CEDICT)
//!
//! By default, no dictionaries are included to keep the binary size small.
//!
//! # Examples
//!
//! ## Basic usage with IPADIC
//!
//! ```toml
//! [dependencies]
//! lindera = "1.4"
//! lindera-tantivy = { version = "1.1", features = ["embedded-ipadic"] }
//! tantivy = "0.25"
//! ```
//!
//! ```rust,no_run
//! use tantivy::{
//!     collector::TopDocs,
//!     doc,
//!     query::QueryParser,
//!     schema::{IndexRecordOption, Schema, TextFieldIndexing, TextOptions},
//!     Index,
//! };
//! use lindera::dictionary::DictionaryKind;
//! use lindera::{dictionary::load_dictionary_from_kind, mode::Mode, segmenter::Segmenter};
//! use lindera_tantivy::tokenizer::LinderaTokenizer;
//!
//! # fn main() -> tantivy::Result<()> {
//! // Create schema
//! let mut schema_builder = Schema::builder();
//! let title = schema_builder.add_text_field(
//!     "title",
//!     TextOptions::default()
//!         .set_indexing_options(
//!             TextFieldIndexing::default()
//!                 .set_tokenizer("lang_ja")
//!                 .set_index_option(IndexRecordOption::WithFreqsAndPositions),
//!         )
//!         .set_stored(),
//! );
//! let schema = schema_builder.build();
//!
//! // Create index
//! let index = Index::create_in_ram(schema.clone());
//!
//! // Create tokenizer with IPADIC
//! let mode = Mode::Normal;
//! let dictionary = load_dictionary_from_kind(DictionaryKind::IPADIC)?;
//! let segmenter = Segmenter::new(mode, dictionary, None);
//! let tokenizer = LinderaTokenizer::from_segmenter(segmenter);
//!
//! // Register tokenizer
//! index.tokenizers().register("lang_ja", tokenizer);
//!
//! // Index documents
//! let mut index_writer = index.writer(50_000_000)?;
//! index_writer.add_document(doc!(title => "東京国際空港"))?;
//! index_writer.commit()?;
//!
//! // Search
//! let reader = index.reader()?;
//! let searcher = reader.searcher();
//! let query_parser = QueryParser::for_index(&index, vec![title]);
//! let query = query_parser.parse_query("東京")?;
//! let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Configuration from YAML file
//!
//! You can also configure the tokenizer using a YAML configuration file:
//!
//! ```rust,no_run
//! use std::path::Path;
//! use lindera_tantivy::tokenizer::LinderaTokenizer;
//!
//! # fn main() -> tantivy::Result<()> {
//! let config_path = Path::new("lindera.yml");
//! let tokenizer = LinderaTokenizer::from_file(config_path)?;
//! # Ok(())
//! # }
//! ```

pub mod stream;
pub mod tokenizer;
