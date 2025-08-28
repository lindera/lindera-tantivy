#[cfg(feature = "embedded-cc-cedict")]
fn main() -> tantivy::Result<()> {
    use tantivy::collector::TopDocs;
    use tantivy::query::QueryParser;
    use tantivy::schema::{IndexRecordOption, Schema, TextFieldIndexing, TextOptions};
    use tantivy::{Document, Index, TantivyDocument, doc};

    use lindera::dictionary::load_dictionary;
    use lindera::mode::Mode;
    use lindera::segmenter::Segmenter;
    use lindera_tantivy::tokenizer::LinderaTokenizer;

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
                    .set_tokenizer("lang_zh")
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
                    .set_tokenizer("lang_zh")
                    .set_index_option(IndexRecordOption::WithFreqsAndPositions),
            )
            .set_stored(),
    );

    // build schema
    let schema = schema_builder.build();

    // create index on memory
    let index = Index::create_in_ram(schema.clone());

    // Tokenizer with CC-CEDICT
    let mode = Mode::Normal;
    let dictionary = load_dictionary("embedded://cc-cedict").unwrap();
    let user_dictionary = None;
    let segmenter = Segmenter::new(mode, dictionary, user_dictionary);
    let tokenizer = LinderaTokenizer::from_segmenter(segmenter);

    // register Lindera tokenizer
    index.tokenizers().register("lang_zh", tokenizer);

    // create index writer
    let mut index_writer = index.writer(50_000_000)?;

    // add document
    index_writer.add_document(doc!(
    id => "1",
    title => "成田国际机场",
    body => "成田國際機場（日语：成田国際空港／なりたこくさいくうこう Narita Kokusai Kūkō */?；IATA代码：NRT；ICAO代码：RJAA），通稱成田機場（成田空港），原名新東京國際機場（新東京国際空港／しんとうきょうこくさいくうこう Shin-Tōkyō Kokusai Kūkō），是位於日本千葉縣成田市的國際機場，與羽田機場並列為東京兩大聯外機場。占地1,111公頃，擁有3座客運航廈，客運流量居日本第二位，貨運吞吐量則居日本第一、全球第九。根據日本機場分類法，其劃分為據點機場。"
    )).unwrap();

    // add document
    index_writer.add_document(doc!(
    id => "2",
    title => "東京國際機場",
    body => "東京國際機場（日语：東京国際空港／とうきょうこくさいくうこう Tōkyō Kokusai Kūkō */?；IATA代码：HND；ICAO代码：RJTT）是位於日本東京都大田區的機場，因座落於羽田地區而通稱為羽田機場（羽田空港／はねだくうこう Haneda Kūkō），啟用於1931年8月25日，與成田國際機場並列為東京兩大聯外機場。"
    )).unwrap();

    // add document
    index_writer.add_document(doc!(
    id => "3",
    title => "关西国际机场",
    body => "關西國際機場（日语：関西国際空港／かんさいこくさいくうこう Kansai kokusai kūkō */?，英語：Kansai International Airport，IATA代码：KIX；ICAO代码：RJBB），常通稱為關西機場、大阪關西機場或關空[註 1]，是位於日本大阪府的機場，坐落於大阪湾东南部的泉州近海離岸5公里的人工島上，面積約1,067.7公頃[2]，行政區劃橫跨大阪府的泉佐野市（北）、田尻町（中）以及泉南市（南）。"
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
    println!("Query String: {query_str}");

    // search
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    println!("Search Result:");
    for (_, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
        println!("{}", retrieved_doc.to_json(&schema));
    }

    Ok(())
}

#[cfg(not(feature = "embedded-cc-cedict"))]
fn main() -> tantivy::Result<()> {
    Ok(())
}
