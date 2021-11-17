use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Encoded {
    pub codes: HuffmanCodes,
    pub tree: HuffmanTree,
    pub encoded_text: String,
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
    pub huffman_code: String,
}

#[derive(Deserialize, Debug)]
pub struct CompressRequest<'a> {
    pub text: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct CompressResponse {
    pub code: Vec<u8>,
}
