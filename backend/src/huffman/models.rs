use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Encoded {
    pub codes: HuffmanCodes,
    pub tree: HuffmanTree,
    pub encoded_text: Vec<Bit>,
}

#[derive(Debug, Serialize)]
pub struct Decoded {
    pub text: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum HuffmanTree {
    Leaf(HuffmanLeaf),
    Node(HuffmanNode),
}

impl HuffmanTree {
    pub fn freq(&self) -> u16 {
        match self {
            HuffmanTree::Leaf(leaf) => leaf.freq,
            HuffmanTree::Node(node) => node.freq,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct HuffmanLeaf {
    pub freq: u16,
    pub value: char,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct HuffmanNode {
    pub freq: u16,
    pub left: Box<HuffmanTree>,
    pub right: Box<HuffmanTree>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Copy)]
#[repr(u8)]
pub enum Bit {
    Zero = b'0',
    One = b'1',
}

impl Into<char> for Bit {
    fn into(self) -> char {
        self as u8 as char
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct HuffmanCodes {
    pub huffman_codes: Vec<HuffmanCode>,
}

impl HuffmanCodes {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HuffmanCode {
    pub character: char,
    pub frequency: u16,
    pub huffman_code: Vec<Bit>,
}

#[derive(Deserialize, Debug)]
pub enum CompressRequest {
    Text(String),
    File, // TODO
}

#[derive(Deserialize, Debug)]
pub struct CompressResponse {
    pub code: Vec<u8>,
}
