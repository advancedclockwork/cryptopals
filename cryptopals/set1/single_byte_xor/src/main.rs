extern crate converter;
extern crate pattern_matcher;
use converter::converter::convert_hex_to_b64;
use pattern_matcher::PatternMatcher;

fn main() {
    let encrypted: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    println!("encrypted string: {}", encrypted);
    pattern_matcher: PatternMatcher = pattern_matcher::new();
    let decrypted: &str = converter::convert_hex_to_b64;

    println!("decrypted string: {}", decrypted);

}
