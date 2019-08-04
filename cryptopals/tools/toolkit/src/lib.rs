pub mod converter{
    extern crate base64;
    extern crate hex;

    pub fn convert_hex_to_b64(to_convert: &str) -> String{
        base64::encode(&hex::decode(to_convert).unwrap())
    }
    #[cfg(test)]
    mod tests {
        use crate::converter::convert_hex_to_b64;

        #[test]
        fn it_works() {
            assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", convert_hex_to_b64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
        }
    }
}
pub mod xor{
    extern crate base64;
    extern crate hex;
    extern crate rayon;
    use rayon::prelude::*;
    pub fn fixed_xor(first_buffer: String, second_buffer: String) -> String{
        let mut to_return: Vec<u8> = vec!();
        let mut index: usize = 0;
        if first_buffer.len() == second_buffer.len() {
            let second_buffer_decoded = hex::decode(second_buffer).unwrap();
            for current_index in hex::decode(first_buffer).unwrap(){
                to_return.push(current_index ^ second_buffer_decoded[index]);
                index += 1;
            }
        } else {
            panic!("both buffers need to be the same length");
        }
        hex::encode(to_return).to_string()
    }
    pub fn single_byte_xor(key: char, to_decrypt: String)->Vec<String>(
        to_decrypt
    )
    #[cfg(test)]
    mod tests {
        use crate::xor::fixed_xor;

        #[test]
        fn fixed_works(){
            assert_eq!("746865206b696420646f6e277420706c6179",fixed_xor("1c0111001f010100061a024b53535009181c".to_string(),"686974207468652062756c6c277320657965".to_string()))
        }
    }
}
