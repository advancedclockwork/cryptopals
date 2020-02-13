use radix_trie::Trie;
use file_reader::file_reader::*;

struct DictionaryTrie {
    viable_words: Trie<String, bool>,
}

impl DictionaryTrie{
    pub fn new() -> DictionaryTrie{
        let word_list: Vec<String> = get_mieliestrok_word_list();
        let mut viable_words: Trie<String, bool> = Trie::new();
        for word in word_list{
            viable_words.insert(word, true);
        }
        DictionaryTrie{ viable_words }
    }

    pub fn check_word(&self, word: String) -> bool {
        match self.viable_words.get(&word){
            None => false,
            Some(_result) => true
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DictionaryTrie;
    #[test]
    fn check_word_test() {
        let dictionary: DictionaryTrie = DictionaryTrie::new();
        assert_eq!(dictionary.check_word("allowance".to_string()), true);
    }
    #[test]
    fn check_word_test_fail() {
        let dictionary: DictionaryTrie = DictionaryTrie::new();
        assert_eq!(dictionary.check_word("adsfasdf".to_string()), false);
    }
}
