# Lindera tokenizer for Tantivy

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Join the chat at https://gitter.im/lindera-morphology/lindera](https://badges.gitter.im/lindera-morphology/lindera.svg)](https://gitter.im/lindera-morphology/lindera?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

[Lindera](https://github.com/lindera-morphology/lindera) Tokenizer for [Tantivy](https://github.com/tantivy-search/tantivy).


## Usage

Make sure you have activated the required dictionaries for the 　Lindera in Cargo.toml.
The following example enables IPADIC.

```
[dependencies]
lindera-tantivy = { version = "0.12.0", features = ["ipadic"] }
```

- ipadic: Japanese dictionary
- unidic: Japanese dictionary
- ko-dic: Korean dictionary
- cc-cedict: Chinese dictionary


### Basic example

```rust
use std::collections::HashSet;

use tantivy::{
    collector::TopDocs,
    doc,
    query::QueryParser,
    schema::{IndexRecordOption, Schema, TextFieldIndexing, TextOptions},
    Index,
};

use lindera::{
    builder,
    character_filter::unicode_normalize::{
        UnicodeNormalizeCharacterFilter, UnicodeNormalizeCharacterFilterConfig,
        UnicodeNormalizeKind,
    },
    mode::Mode,
    token_filter::{
        japanese_compound_word::{
            JapaneseCompoundWordTokenFilter, JapaneseCompoundWordTokenFilterConfig,
        },
        japanese_number::{JapaneseNumberTokenFilter, JapaneseNumberTokenFilterConfig},
    },
    tokenizer::Tokenizer,
    BoxCharacterFilter, BoxTokenFilter, DictionaryKind,
};
use lindera_tantivy::tokenizer::LinderaTokenizer;

fn main() -> tantivy::Result<()> {
    // create schema builder
    let mut schema_builder = Schema::builder();

    // add id field
    let id = schema_builder.add_text_field(
        "id",
        TextOptions::default()
            .set_indexing_options(
                TextFieldIndexing::default()
                    .set_tokenizer("raw")
                    .set_index_option(IndexRecordOption::Basic),
            )
            .set_stored(),
    );

    // add title field
    let title = schema_builder.add_text_field(
        "title",
        TextOptions::default()
            .set_indexing_options(
                TextFieldIndexing::default()
                    .set_tokenizer("lang_ja")
                    .set_index_option(IndexRecordOption::WithFreqsAndPositions),
            )
            .set_stored(),
    );

    // add body field
    let body = schema_builder.add_text_field(
        "body",
        TextOptions::default()
            .set_indexing_options(
                TextFieldIndexing::default()
                    .set_tokenizer("lang_ja")
                    .set_index_option(IndexRecordOption::WithFreqsAndPositions),
            )
            .set_stored(),
    );

    // build schema
    let schema = schema_builder.build();

    // create index on memory
    let index = Index::create_in_ram(schema.clone());

    // Character filters.
    // Filters are executed in the order in which they are added.
    // Character filters are performed before the text is tokenized in the tokenizer.
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

    // Token filters.
    // Filters are executed in the order in which they are added.
    // Token filters are performed after the text is tokenized in the tokenizer.
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

    // register Lindera tokenizer
    index
        .tokenizers()
        .register("lang_ja", LinderaTokenizer::new(character_filters, tokenizer, token_filters));

    // create index writer
    let mut index_writer = index.writer(50_000_000)?;

    // add document
    index_writer.add_document(doc!(
    id => "1",
    title => "成田国際空港",
    body => "成田国際空港（なりたこくさいくうこう、英: Narita International Airport）は、千葉県成田市南東部から芝山町北部にかけて建設された日本最大の国際拠点空港である。首都圏東部（東京の東60km）に位置している。空港コードはNRT。"
    )).unwrap();

    // add document
    index_writer.add_document(doc!(
    id => "2",
    title => "東京国際空港",
    body => "東京国際空港（とうきょうこくさいくうこう、英語: Tokyo International Airport）は、東京都大田区にある日本最大の空港。通称は羽田空港（はねだくうこう、英語: Haneda Airport）であり、単に「羽田」と呼ばれる場合もある。空港コードはHND。"
    )).unwrap();

    // add document
    index_writer.add_document(doc!(
    id => "3",
    title => "関西国際空港",
    body => "関西国際空港（かんさいこくさいくうこう、英: Kansai International Airport）は大阪市の南西35㎞に位置する西日本の国際的な玄関口であり、関西三空港の一つとして大阪国際空港（伊丹空港）、神戸空港とともに関西エアポート株式会社によって一体運営が行われている。"
    )).unwrap();

    // commit
    index_writer.commit()?;

    // create reader
    let reader = index.reader()?;

    // create searcher
    let searcher = reader.searcher();

    // create querhy parser
    let query_parser = QueryParser::for_index(&index, vec![title, body]);

    // parse query
    let query_str = "東京";
    let query = query_parser.parse_query(query_str)?;
    println!("Query String: {}", query_str);

    // search
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    println!("Search Result:");
    for (_, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        println!("{}", schema.to_json(&retrieved_doc));
    }

    Ok(())
}
```

## API reference

The API reference is available. Please see following URL:
- <a href="https://docs.rs/lindera-tantivy" target="_blank">lindera-tantivy</a>
