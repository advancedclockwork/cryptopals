use dictionary_trie;
use file_reader;
use pattern_matcher;
fn main() {
    const ENCRYPTED_STRINGS_LOCATION = "./resources/encrypted.txt"
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
    let all_words = dictionary_trie::DictionaryTrie::new();
    let common_letters = file_reader::get_most_common_letters();
    let encrypted_strings = file_reader::get_file_contents(ENCRYPTED_STRINGS_LOCATION);
    let pattern_matcher = pattern_matcher::PatternMatcher::new();
    let mut score = 0;
    let mut high_score_string = "";
    let mut decrypted_high_score_string = "";
    let mut line_number = 0;
    for encrypted_string in encrypted_strings{
        let most_common_letters_in_encrypted_string = pattern_matcher.find_most_common_char_order(encrypted_string);
    }
    
}
