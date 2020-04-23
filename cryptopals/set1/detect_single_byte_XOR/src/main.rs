extern crate dictionary_trie;
extern crate file_reader;
extern crate pattern_matcher;
extern crate toolkit;
use pattern_matcher::pattern_matcher::PatternMatcher;
use dictionary_trie::DictionaryTrie;
fn main() {
    const ENCRYPTED_STRINGS_LOCATION: &str = "./resources/encrypted.txt";
    //read list of all words
    //read list of most common char order
    //read list of encrypted strings
    //store score, high score string, decrypted high score string
    //for encrypted string in encrypted strings
        //find most common char in encrypted string
        //for current char in most common char order
            //decrypted string = xor each char in encrypted string by most common char
            //split decrypted string by space chars
            //current score = 0
            //for each string in split decrypted string
                //if compare with valid words
                    //current score++
            //if current score > score
                //score = current score
                //high score string = encrypted string
                //decrypted high score string = decrypted string
    let all_words: DictionaryTrie = DictionaryTrie::new();
    let common_letters: Vec<String> = file_reader::file_reader::get_most_common_letters();
    let encrypted_strings: Vec<String> = file_reader::file_reader::get_file_contents(ENCRYPTED_STRINGS_LOCATION);
    let pattern_matcher: PatternMatcher = PatternMatcher::new();
    let mut high_score = 0;
    let mut high_score_string = "".to_string();
    let mut decrypted_high_score_string = "".to_string();
    let mut high_score_line_number = 0;
    let mut line_number = 0;
    for encrypted_string in encrypted_strings{
        let mut score = 0;
        let most_common_letters_in_encrypted_string = pattern_matcher.find_most_common_char_order(&encrypted_string);
        for entry: char in common_letters{//common letters is the issue
            let key = entry ^ most_common_letters_in_encrypted_string[0].0;
            let decrypted_string = toolkit::xor::single_byte_xor(key,encrypted_string.to_string());
            for word in decrypted_string.split(" "){
                if all_words.check_word(word.to_string()){
                    score += 1;
                }
            }
            //if score > high_score{
            //    high_score = score;
            //    high_score_string = (&encrypted_string).clone();
            //    decrypted_high_score_string = (&decrypted_string).clone();
            //    high_score_line_number = line_number;
            //}
        }
        line_number += 1;
    }
    print!("Best Match:\nHigh Score: {}\nEncrypted String: {}\nDecrypted String: {}\nLine Number: {}", high_score, high_score_string, decrypted_high_score_string, high_score_line_number);
    
}
