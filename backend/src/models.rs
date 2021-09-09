use serde::{Deserialize, Serialize};

#[derive(Serialize, Default)]
pub struct HuffmanCodes {
    pub huffman_codes: Vec<Option<HuffmanCode>>,
}

impl HuffmanCodes {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct HuffmanCode {
    pub character: char,
    pub frequency: u32,
    pub huffman_code: Vec<u8>,
}

#[derive(Deserialize, Debug)]
pub struct TextData<'a> {
    pub text: &'a str,
}
