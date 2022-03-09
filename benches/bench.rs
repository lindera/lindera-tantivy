use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

#[cfg(feature = "ipadic")]
fn bench_indexing(c: &mut Criterion) {
    use tantivy::doc;
    use tantivy::schema::IndexRecordOption;
    use tantivy::schema::Schema;
    use tantivy::schema::TextFieldIndexing;
    use tantivy::schema::TextOptions;
    use tantivy::Index;

    use lindera::tokenizer::{DictionaryType, TokenizerConfig, UserDictionaryType};
    use lindera_core::viterbi::{Mode, Penalty};
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

    // add text field
    let text = schema_builder.add_text_field(
        "text",
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
        dict_type: DictionaryType::Ipadic,
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
    let mut index_writer = index.writer(50_000_000).unwrap();

    // Using benchmark_group for changing sample_size
    let mut group = c.benchmark_group("indexing");
    group.sample_size(100);
    group.bench_function("bench-indexing", |b| {
        b.iter(|| {
            // add document
            index_writer.add_document(doc!(
                id => "1",
                text => "成田国際空港（なりたこくさいくうこう、英: Narita International Airport）は、千葉県成田市南東部から芝山町北部にかけて建設された日本最大の国際拠点空港である[1]。首都圏東部（東京の東60km）に位置している。空港コードはNRT。"
                ))
        });

        // commit
        index_writer.commit().unwrap();
    });
    group.finish();
}

#[cfg(not(feature = "ipadic"))]
fn bench_indexing(_c: &mut Criterion) {}

criterion_group!(benches, bench_indexing,);
criterion_main!(benches);
