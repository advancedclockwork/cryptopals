extern crate toolkit;
use toolkit::xor::fixed_xor;
use std::string::ToString;

fn main() {
    println!("intended result: {}", "746865206b696420646f6e277420706c6179");
    println!("found result:    {}", fixed_xor("1c0111001f010100061a024b53535009181c".to_string(),"686974207468652062756c6c277320657965".to_string()));
}
