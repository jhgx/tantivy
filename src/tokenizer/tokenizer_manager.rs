use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokenizer::BoxedTokenizer;
use tokenizer::Tokenizer;
use tokenizer::tokenizer::box_tokenizer;
use tokenizer::RawTokenizer;
use tokenizer::SimpleTokenizer;
use tokenizer::JapaneseTokenizer;
use tokenizer::RemoveLongFilter;
use tokenizer::LowerCaser;
use tokenizer::Stemmer;



/// The tokenizer manager serves as a store for
/// all of the pre-configured tokenizer pipelines.
///
/// By default, it is populated with the following managers.
///
///  * raw : does not process nor tokenize the text.
///  * default : Chops the text on according to whitespace and
///  punctuation, removes tokens that are too long, lowercases
#[derive(Clone)]
pub struct TokenizerManager {
    tokenizers: Arc<RwLock<HashMap<String, Box<BoxedTokenizer>>>>,
}

impl TokenizerManager {
    pub fn register<A>(&self, tokenizer_name: &str, tokenizer: A)
    where
        A: 'static + Send + Sync + for<'a> Tokenizer<'a>,
    {
        let boxed_tokenizer = box_tokenizer(tokenizer);
        self.tokenizers
            .write()
            .expect("Acquiring the lock should never fail")
            .insert(tokenizer_name.to_string(), boxed_tokenizer);
    }

    pub fn get(&self, tokenizer_name: &str) -> Option<Box<BoxedTokenizer>> {
        self.tokenizers
            .read()
            .expect("Acquiring the lock should never fail")
            .get(tokenizer_name)
            .map(|boxed_tokenizer| boxed_tokenizer.boxed_clone())
    }
}

impl Default for TokenizerManager {
    /// Creates an `TokenizerManager` prepopulated with
    /// the default pre-configured tokenizers of `tantivy`.
    /// - simple
    /// - en_stem
    /// - ja
    fn default() -> TokenizerManager {
        let manager = TokenizerManager { tokenizers: Arc::new(RwLock::new(HashMap::new())) };
        manager.register("raw", RawTokenizer);
        manager.register(
            "default",
            SimpleTokenizer.filter(RemoveLongFilter::limit(40)).filter(
                LowerCaser,
            ),
        );
        manager.register(
            "en_stem",
            SimpleTokenizer
                .filter(RemoveLongFilter::limit(40))
                .filter(LowerCaser)
                .filter(Stemmer::new()),
        );
        manager.register("ja", JapaneseTokenizer.filter(RemoveLongFilter::limit(40)));
        manager
    }
}
