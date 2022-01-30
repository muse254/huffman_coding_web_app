//! This module containing all the models used in the decoding and decoding processes.
use serde::{Deserialize, Serialize};

/// Encoded represent the Encode form of the text provided from input for
/// compression. It's what the API returns on successful encoding.
#[derive(Debug, Deserialize, Serialize)]
pub struct Encoded {
    pub codes: HuffmanCodes,
    pub tree: HuffmanTree,
    pub encoded_text: String,
}

/// Decoded wraps the text that is as a result of the decoding operation.
/// It's what the API returns on successful decoding.
#[derive(Debug, Serialize)]
pub struct Decoded {
    pub text: String,
}

/// The HuffmanTree contains the Huffman Tree structure.
///
/// The HuffmanTree is a binary tree that contains the Huffman Codes.
/// The HuffmanTree is either a leaf or a node.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum HuffmanTree {
    Leaf(HuffmanLeaf),
    Node(HuffmanNode),
}

impl HuffmanTree {
    /// This returns the frequency of the tree.
    pub fn freq(&self) -> u64 {
        match self {
            HuffmanTree::Leaf(leaf) => leaf.freq,
            HuffmanTree::Node(node) => node.freq,
        }
    }

    /// This checks whether the tree contains only a single node(leaf).
    pub fn is_single_node(&self) -> bool {
        match self {
            HuffmanTree::Leaf(_) => true,
            HuffmanTree::Node(_) => false,
        }
    }

    /// This retrieves the value of the single node in the tree.
    pub fn get_single_node_value(&self) -> char {
        match self {
            HuffmanTree::Node(_) => panic!("This is not a single node tree"),
            HuffmanTree::Leaf(leaf) => leaf.value,
        }
    }

    /// This retrieves the frequency value of the single node in the tree.
    ///
    /// This method is redundant with the `freq` method but it asserts that
    /// the tree is a single node tree.
    pub fn get_single_node_frequency(&self) -> u64 {
        match self {
            HuffmanTree::Node(_) => panic!("This is not a single node tree"),
            HuffmanTree::Leaf(leaf) => leaf.freq,
        }
    }
}

/// The HuffamnLeaf contains the Huffman Leaf structure.
///
/// The `freq` field is the frequency of the leaf.
/// The `value` field is the value of the leaf.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct HuffmanLeaf {
    pub freq: u64,
    pub value: char,
}

/// The HuffmanNode contains the Huffman Node structure.
///
/// The `left` and `right` fields are the left and right children of the node.
/// The `freq` field is the frequency of the node.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct HuffmanNode {
    pub freq: u64,
    pub left: Box<HuffmanTree>,
    pub right: Box<HuffmanTree>,
}

/// The HuffmanCodes contains the Huffman Codes collection.
///
/// The `huffman_codes` field is the collection of Huffman Codes.
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct HuffmanCodes {
    pub huffman_codes: Vec<HuffmanCode>,
}

impl HuffmanCodes {
    pub fn new() -> Self {
        Default::default()
    }
}

/// HuffmanCode wraps the character specofic information after successfully encoding
/// text provided from input.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HuffmanCode {
    pub character: char,
    pub frequency: u64,
    pub huffman_code: String,
}

/// This wraps the text from tha API input that has been sent for compression.
#[derive(Deserialize, Debug)]
pub struct CompressRequest {
    pub text: String,
}

/// This wraps the compression response from the encoding module for the text provided
/// for compression from input.
#[derive(Deserialize, Debug)]
pub struct CompressResponse {
    pub code: Vec<u8>,
}
