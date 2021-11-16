use serde::{Deserialize, Serialize};

#[derive(PartialEq)]
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

#[derive(PartialEq)]
pub struct HuffmanLeaf {
    pub freq: u16,
    pub value: char,
}

#[derive(PartialEq)]
pub struct HuffmanNode {
    pub freq: u16,
    pub left: Box<HuffmanTree>,
    pub right: Box<HuffmanTree>,
}

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
