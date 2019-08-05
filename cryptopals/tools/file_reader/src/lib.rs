pub mod file_reader {
    use std::fs::File;
    use std::path::Path;
    use std::path::Display;
    use std::error::Error;
    use std::io::{BufReader, Read};

    pub fn get_values(file_path: &str) -> Vec<String>{
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
        let mut file_contents= String::new();
        match reader.read_to_string(&mut file_contents){
            Err(why) => panic!("file could not be read correctly read: {}",
                               why.description()),
            _ => (), //do nothing
        };
        file_contents.split("\n").map(|s| s.to_string()).collect::<Vec<String>>()
    }
    #[cfg(test)]
    mod tests {
        use crate::file_reader::get_values;
        #[test]
        fn it_works() {
            assert_eq!(&"asdf".to_string(), get_values(
                "/home/owen/github projects/cryptopals/cryptopals/tools/file_reader/test/test")
                .get(1).unwrap());
        }
    }
}