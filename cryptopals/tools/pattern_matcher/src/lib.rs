pub mod pattern_matcher{
    use std::collections::HashMap;
    use dictionary_trie::DictionaryTrie;
    use converter;
    use file_reader;

    pub struct PatternMatcher{
        to_decrypt                 : String,
        key                        : char,
        number_of_valid_words      : i32,
        possible_decrypted_string  : String,
        dictionary_trie            : DictionaryTrie,
        most_common_char_order     : Vec<(char,i32)>,
    }

    impl PatternMatcher{
        pub fn new(to_decrypt: String, dictionary_trie: DictionaryTrie, possible_key) -> PatternMatcher{
            let to_decrypt_b64 = converter.convert_hex_to_b64(to_decrypt);
            let most_common_char_order = determine_most_common_char_order();
            
            let 
            let key = find_possible_key();
            PatternMatcher{ to_decrypt,
                            possible_key,
                            dictionary_trie,
                            most_common_char_order() }
        }
        fn find_possible_key(possible_key, char_to_test)
        fn decrypt(to_decrypt, )

        fn determine_most_common_char_order(&self) -> Vec<(char, i32)>{
            let mut character_frequency_store: HashMap<char,i32> = HashMap::new();
            for character in self.to_decrypt.chars(){
                let entry = character_frequency_store.entry(character).or_insert(0);
                *entry += 1;
            }
            let mut character_frequency: Vec<(char, i32)> = vec!();
            for (key,val) in character_frequency_store {
                character_frequency.push((key,val));
            }
            character_frequency.sort_by(|a, b| b.1.cmp(&a.1));
            character_frequency
        }
    }


    #[cfg(test)]
    mod tests {
        use crate::pattern_matcher::PatternMatcher;
        use dictionary_trie::DictionaryTrie;

        #[test]
        fn pattern_match_test() {
            let dictionary_trie: DictionaryTrie = DictionaryTrie::new();
            let pattern_matcher: PatternMatcher = PatternMatcher::new("aaabbbbbbb".to_string(), dictionary_trie);
            assert_eq!(('b', 7), pattern_matcher.find_most_common_char_order()[0]);
        }
    }
}