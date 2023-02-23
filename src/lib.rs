pub mod dictionary;
pub mod stream;
pub mod tokenizer;

pub type LinderaResult<T> = lindera::LinderaResult<T>;
pub type Penalty = lindera::mode::Penalty;
pub type Mode = lindera::mode::Mode;
pub type DictionaryConfig = lindera::dictionary::DictionaryConfig;
pub type UserDictionryConfig = lindera::dictionary::UserDictionaryConfig;
pub type DictionaryKind = lindera::DictionaryKind;
pub type Dictionary = lindera::Dictionary;
pub type UserDictionary = lindera::UserDictionary;
