pub mod file_reader {
    use std::fs::File;
    use std::path::Path;
    use std::path::Display;
    use std::io::{BufReader, Read};
    use toml;
    use serde_derive::Deserialize;

    #[derive(Deserialize)]
    pub struct Paths {
        pub directory                       : String,
        pub mieliestrok_word_list           : String,
        pub most_common_digraphs            : String,
        pub most_common_doubles             : String,
        pub most_common_four_letter_words   : String,
        pub most_common_initial_letter      : String,
        pub most_common_last_letter         : String,
        pub most_common_letters             : String,
        pub most_common_one_letter_words    : String,
        pub most_common_three_letter_words  : String,
        pub most_common_trigraphs           : String,
        pub most_common_two_letter_words    : String,
    }

    impl Paths{
        pub fn new(){

        }
    }

    /// Attempt to load and parse the config file into our Config struct.
    /// If a file cannot be found, return a default Config.
    /// If we find a file but cannot parse it, panic
    pub fn get_path_values() -> Paths {
        const PATH_FILE: &str = "../../tools/file_reader/config/Paths_to_crypto_data.toml";

        let mut config_toml = String::new();

        let mut file = match File::open(&PATH_FILE) {
            Ok(file) => file,
            Err(_)  => {
                panic!("Could not find config file at {}", PATH_FILE);
            }
        };

        file.read_to_string(&mut config_toml)
                .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

        match toml::from_str(&config_toml) {
            Ok(t) => t,
            Err(why) => panic!("values for {} could not be parsed: {}", PATH_FILE, why.to_string())
        }
    }

    pub fn get_mieliestrok_word_list()-> Vec<String>{
        let paths = get_path_values();
        get_file_contents(&format!("{}{}",paths.directory,paths.mieliestrok_word_list))
    }

    pub fn get_most_common_digraphs()-> Vec<String>{
        let paths = get_path_values();
        get_file_contents(&format!("{}{}",paths.directory,paths.most_common_digraphs))
    }

    pub fn get_most_common_doubles()-> Vec<String>{
        let paths = get_path_values();
        get_file_contents(&format!("{}{}",paths.directory,paths.most_common_doubles))
    }

    pub fn get_most_common_four_letter_words()-> Vec<String>{
        let paths = get_path_values();
        get_file_contents(&format!("{}{}",paths.directory,paths.most_common_four_letter_words))
    }

    pub fn get_most_common_initial_letter()-> Vec<String>{
        let paths = get_path_values();
        get_file_contents(&format!("{}{}",paths.directory,paths.most_common_initial_letter))
    }

    pub fn get_most_common_last_letter()-> Vec<String>{
        let paths = get_path_values();
        get_file_contents(&format!("{}{}",paths.directory,paths.most_common_last_letter))
    }

    pub fn get_most_common_letters()-> Vec<String>{
        let paths = get_path_values();
        get_file_contents(&format!("{}{}",paths.directory,paths.most_common_letters))
    }

    pub fn get_most_common_one_letter_words()-> Vec<String>{
        let paths = get_path_values();
        get_file_contents(&format!("{}{}",paths.directory,paths.most_common_one_letter_words))
    }

    pub fn get_most_common_three_letter_words()-> Vec<String>{
        let paths = get_path_values();
        get_file_contents(&format!("{}{}",paths.directory,paths.most_common_three_letter_words))
    }

    pub fn get_most_common_trigraphs()-> Vec<String>{
        let paths = get_path_values();
        get_file_contents(&format!("{}{}",paths.directory,paths.most_common_trigraphs))
    }

    pub fn get_most_common_two_letter_words()-> Vec<String>{
        let paths = get_path_values();
        get_file_contents(&format!("{}{}",paths.directory,paths.most_common_two_letter_words))
    }

    pub fn get_file_contents(file_path: &str) -> Vec<String>{
        get_values(&file_path).split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()
    }

    fn get_values(file_path: &str) -> String{
        let path:&Path = Path::new(file_path);
        let display:Display = path.display();

        let file: File = match File::open(path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display,
                               why.to_string()),
            Ok(file) => file,
        };
        let mut reader = BufReader::new(file);
        let mut file_contents = String::new();
        match reader.read_to_string(&mut file_contents){
            Err(why) => panic!("file {} could not be read correctly: {}", file_path,
                               why.to_string()),
            _ => (),
        };
        file_contents
    }

    #[cfg(test)]
    mod tests {
        use crate::file_reader::*;
        #[test]
        fn get_file_contents_test() {
            assert_eq!(&"asdf".to_string(), get_file_contents(
                "./test/test")
                .get(1).unwrap());
        }

        #[test]
        fn get_path_value_test() {
            assert_eq!("MostCommonDigraphsEnglish".to_string(), get_path_values().most_common_digraphs);
        }

        #[test]
        fn get_mieliestrok_word_list_test(){
            assert_eq!(true, get_mieliestrok_word_list().contains(&"aardvark".to_string()));
        }

        #[test]
        fn get_most_common_digraphs_test(){
            assert_eq!(true, get_most_common_digraphs().contains(&"of".to_string()));
        }

        #[test]
        fn get_most_common_doubles_test(){
            assert_eq!(true, get_most_common_doubles().contains(&"ff".to_string()));
        }

        #[test]
        fn get_most_common_four_letter_words_test(){
            assert_eq!(true, get_most_common_four_letter_words().contains(&"know".to_string()));
        }

        #[test]
        fn get_most_common_initial_letter_test(){
            assert_eq!(true, get_most_common_initial_letter().contains(&"R".to_string()));
        }

        #[test]
        fn get_most_common_last_letter_test(){
            assert_eq!(true, get_most_common_last_letter().contains(&"G".to_string()));
        }

        #[test]
        fn get_most_common_letters_test(){
            assert_eq!(true, get_most_common_letters().contains(&"S".to_string()));
        }

        #[test]
        fn get_most_common_one_letter_words_test(){
            assert_eq!(true, get_most_common_one_letter_words().contains(&"I".to_string()));
        }

        #[test]
        fn get_most_common_three_letter_words_test(){
            assert_eq!(true, get_most_common_three_letter_words().contains(&"him".to_string()));
        }

        #[test]
        fn get_most_common_trigraphs_test(){
            assert_eq!(true, get_most_common_trigraphs().contains(&"nce".to_string()));
        }

        #[test]
        fn get_most_common_two_letter_words_test(){
            assert_eq!(true, get_most_common_two_letter_words().contains(&"or".to_string()));
        }

    }
}