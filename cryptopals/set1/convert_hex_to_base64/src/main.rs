extern crate converter;
use converter::converter::convert_hex_to_b64;
fn main() {
    println!("{}",convert_hex_to_b64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
}
