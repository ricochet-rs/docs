use crate::docs::{DOC_PAGES, DocPage};
use bm25::{
    DefaultTokenizer, Document, Language, LanguageMode, SearchEngine, SearchEngineBuilder,
    SearchResult,
};
use std::sync::OnceLock;
static SEARCH_ENGINE: OnceLock<SearchEngine<u32>> = OnceLock::new();

impl From<DocPage> for Document<String> {
    fn from(value: DocPage) -> Self {
        Document::new(value.title.to_string(), value.body.to_string())
    }
}

// Create the search engine
pub fn search_engine() -> &'static SearchEngine<u32> {
    SEARCH_ENGINE.get_or_init(|| {
        SearchEngineBuilder::<u32>::with_tokenizer_and_corpus(
            DefaultTokenizer::new(LanguageMode::Fixed(Language::English)),
            DOC_PAGES.map(|i| i.body),
        )
        .build()
    })
}

// Query the contents of the engine
pub fn query_engine(query: &str, n: Option<usize>) -> Vec<SearchResult<u32>> {
    let engine = search_engine();
    engine.search(query, n)
}
