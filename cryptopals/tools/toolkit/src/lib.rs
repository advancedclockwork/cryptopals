pub mod xor{
    extern crate base64;
    extern crate hex;

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

    pub fn single_byte_xor(key: char, to_decrypt: String)->String{
        let mut to_return: Vec<u8> = vec!();
        for current_index in hex::decode(to_decrypt).unwrap(){
            to_return.push(current_index ^ hex::decode(key.to_string()).unwrap()[0]);
        }
        hex::encode(to_return).to_string()
    }

    #[cfg(test)]
    mod tests {
        use crate::xor::fixed_xor;

        #[test]
        fn fixed_works(){
            assert_eq!("746865206b696420646f6e277420706c6179",fixed_xor("1c0111001f010100061a024b53535009181c".to_string(),"686974207468652062756c6c277320657965".to_string()))
        }
    }
}
