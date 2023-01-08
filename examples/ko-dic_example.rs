#[cfg(feature = "ko-dic")]
fn main() -> tantivy::Result<()> {
    use tantivy::collector::TopDocs;
    use tantivy::doc;
    use tantivy::query::QueryParser;
    use tantivy::schema::{IndexRecordOption, Schema, TextFieldIndexing, TextOptions};
    use tantivy::Index;

    use lindera_tantivy::mode::Mode;
    use lindera_tantivy::tokenizer::{
        DictionaryConfig, DictionaryKind, LinderaTokenizer, TokenizerConfig,
    };

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
                    .set_tokenizer("lang_ko")
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
                    .set_tokenizer("lang_ko")
                    .set_index_option(IndexRecordOption::WithFreqsAndPositions),
            )
            .set_stored(),
    );

    // build schema
    let schema = schema_builder.build();

    // create index on memory
    let index = Index::create_in_ram(schema.clone());

    let dictionary = DictionaryConfig {
        kind: Some(DictionaryKind::KoDic),
        path: None,
    };

    let config = TokenizerConfig {
        dictionary,
        user_dictionary: None,
        mode: Mode::Normal,
        with_details: false,
    };

    // register Lindera tokenizer
    index
        .tokenizers()
        .register("lang_ko", LinderaTokenizer::from_config(config).unwrap());

    // create index writer
    let mut index_writer = index.writer(50_000_000)?;

    // add document
    index_writer.add_document(doc!(
    id => "1",
    title => "나리타 국제공항",
    body => "나리타 국제공항(일본어: 成田国際空港, 영어: Narita International Airport, IATA: NRT, ICAO: RJAA)은 일본 지바현 나리타시에 위치한 국제공항으로, 도쿄도 도심에서 동북쪽으로 약 62km 떨어져 있다."
    )).unwrap();

    // add document
    index_writer.add_document(doc!(
    id => "2",
    title => "도쿄 국제공항",
    body => "도쿄국제공항(일본어: 東京国際空港、とうきょうこくさいくうこう, 영어: Tokyo International Airport)은 일본 도쿄도 오타구에 있는 공항이다. 보통 이 일대의 옛 지명을 본뜬 하네다 공항(일본어: 羽田空港, 영어: Haneda Airport)이라고 불린다."
    )).unwrap();

    // add document
    index_writer.add_document(doc!(
    id => "3",
    title => "간사이 국제공항",
    body => "간사이 국제공항(일본어: 関西国際空港, IATA: KIX, ICAO: RJBB)은 일본 오사카부 오사카 만에 조성된 인공섬에 위치한 일본의 공항으로, 대한민국의 인천국제공항보다 6년 반 앞선 1994년 9월 4일에 개항했다."
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
    let query_str = "도쿄";
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

#[cfg(not(feature = "ko-dic"))]
fn main() -> tantivy::Result<()> {
    Ok(())
}
