mod pattern_matcher {
    use std::collections::HashMap;
    use dictionary_trie;
    use file_reader;
    struct PatternMatcher{
        common_chars: Vec<(char, i32)>,
        all_words: DictionaryTrie,
    }

    impl PatternMatcher{
        fn new() -> PatternMatcher{
            let common_chars = find_most_common_char_order(string_to_check);
            let all_words = dictionary_trie::DictionaryTrie::new();
        }
        //pub fn simple_pattern_match(encrypted_string: &str, key: &str) -> String {
        //    let most_common_char_order: Vec<i8> = find_most_common_char(encrypted_string);
        //}
        pub fn find_most_likely_decryption(&self, string_to_decrypt: String){

        }


        fn find_most_common_char_order(&self, string_to_check: &str) -> Vec<(char, i32)>{
            let mut character_frequency_store: HashMap<char,i32> = HashMap::new();
            for character in string_to_check.chars(){
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
        use crate::pattern_matcher::find_most_common_char_order;
        #[test]
        fn pattern_match_test() {
            assert_eq!(('b', 7), find_most_common_char_order("aaabbbbbbb")[0]);
        }
    }
}