use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Clone, Debug)]
pub struct HuffmanCodes {
    pub huffman_codes: Vec<HuffmanCode>,
}

impl HuffmanCodes {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct HuffmanCode {
    pub character: char,
    pub frequency: u16,
    pub huffman_code: String,
}

#[derive(Deserialize, Debug)]
pub struct CompressRequest<'a> {
    pub text: &'a str,
}
