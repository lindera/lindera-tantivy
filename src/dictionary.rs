use crate::{Dictionary, DictionaryConfig, LinderaResult};

pub fn load_dictionary(dictionary_config: DictionaryConfig) -> LinderaResult<Dictionary> {
    lindera::dictionary::load_dictionary(dictionary_config)
}
