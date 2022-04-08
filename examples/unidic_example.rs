#[cfg(feature = "unidic")]
fn main() -> tantivy::Result<()> {
    use tantivy::collector::TopDocs;
    use tantivy::doc;
    use tantivy::query::QueryParser;
    use tantivy::schema::{IndexRecordOption, Schema, TextFieldIndexing, TextOptions};
    use tantivy::Index;

    use lindera_tantivy::mode::{Mode, Penalty};
    use lindera_tantivy::tokenizer::LinderaTokenizer;
    use lindera_tantivy::tokenizer::{DictionaryType, TokenizerConfig, UserDictionaryType};

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

    let config = TokenizerConfig {
        dict_type: DictionaryType::Unidic,
        dict_path: None,
        user_dict_path: None,
        user_dict_type: UserDictionaryType::Csv,
        mode: Mode::Decompose(Penalty::default()),
    };

    // register Lindera tokenizer
    index
        .tokenizers()
        .register("lang_ja", LinderaTokenizer::with_config(config).unwrap());

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

#[cfg(not(feature = "unidic"))]
fn main() -> tantivy::Result<()> {
    Ok(())
}
