
/// the converter module is intended to handle conversions between data types
pub mod converter{
    extern crate base64;
    extern crate hex;

    /// convert_hex_to_b64 takes hex string slice and creates a base64 string and returns the base64 string
    pub fn convert_hex_to_b64(to_convert: &str) -> String{
        base64::encode(&hex::decode(to_convert).unwrap())
    }

    /// convert_b64_to_hex takes b64 string slice and creates a hex string and returns the hex string
    pub fn convert_b64_to_hex(to_convert: &str) -> String{
        hex::encode(&base64::decode(to_convert).unwrap())        
    }

    

    #[cfg(test)]
    mod tests {
        use crate::converter::convert_hex_to_b64;
        use crate::converter::convert_b64_to_hex;

        #[test]
        fn convert_hex_to_b64_test_str() {
            assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
                        convert_hex_to_b64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
        }

        #[test]
        fn convert_b64_to_hex_test() {
            assert_eq!("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
                        convert_b64_to_hex("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"));
        }
    }
}