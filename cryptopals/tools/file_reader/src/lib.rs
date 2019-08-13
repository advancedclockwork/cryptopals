pub mod file_reader {
    use std::fs::File;
    use std::path::Path;
    use std::path::Display;
    use std::error::Error;
    use std::io::{BufReader, Read};
    use toml;
    use serde_derive::Deserialize;

    #[derive(Deserialize)]
    pub struct Paths {
        directory                       : String,
        mieliestrok_work_list           : String,
        most_common_digraphs            : String,
        most_common_doubles             : String,
        most_common_four_letter_words   : String,
        most_common_initial_letter      : String,
        most_common_last_letter         : String,
        most_common_letters             : String,
        most_common_one_letter_words    : String,
        most_common_three_letter_words  : String,
        most_common_trigraphs           : String,
        most_common_two_letter_words    : String,
    }

    pub fn get_path_values(file_path: &str) -> Paths {
        match toml::from_str(&get_values(file_path)) {
            Err(why) => panic!("values for {} could not be parsed: {}", file_path, why.description()),
            Ok(paths) => paths, //do nothing
        }
    }

    pub fn get_file_contents(file_path: &str) -> Vec<String>{
        get_values(&file_path).split("\n").map(|s| s.to_string()).collect::<Vec<String>>()
    }

    fn get_values(file_path: &str) -> String{
        let path:&Path = Path::new(file_path);
        let display:Display = path.display();

        let file: File = match File::open(path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display,
                               why.description()),
            Ok(file) => file,
        };
        let mut reader = BufReader::new(file);
        let mut file_contents = String::new();
        match reader.read_to_string(&mut file_contents){
            Err(why) => panic!("file {} could not be read correctly: {}", file_path,
                               why.description()),
            _ => (),
        };
        file_contents
    }

    #[cfg(test)]
    mod tests {
        use crate::file_reader::*;
        #[test]
        fn get_file_contents_works() {
            assert_eq!(&"asdf".to_string(), get_file_contents(
                "./test/test")
                .get(1).unwrap());
        }

        #[test]
        fn get_path_value_works() {
            assert_eq!("MostCommonDigraphsEnglish".to_string(), get_path_values("./config/Paths_to_crypto_data.toml").most_common_digraphs);
        }
    }
}