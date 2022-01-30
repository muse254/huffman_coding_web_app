//! This module containing all the models used in the decoding and decoding processes.
use axum::{http::StatusCode, response::IntoResponse, Json};
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HuffmanCode {
    pub character: char,
    pub frequency: u64,
    pub huffman_code: String,
}

#[derive(Deserialize, Debug)]
pub struct CompressRequest {
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct CompressResponse {
    pub code: Vec<u8>,
}

/// AppError allows to wrap errors and return as responses to a server request.
#[derive(Debug, Serialize)]
pub enum AppError {
    /// BadRequest is used when the request body's request cannot be parsed.
    BadRequest(String),
    /// InternalServerError is returned when there's an issue while processing
    /// the client request.
    InternalServerError(String),
    /// NotFound is returned when the resource requested for by the client request
    /// was not found.
    NotFound(String),
}

/// IntoResponse trait is implemented for AppError in order to be able
/// to return AppError as a server response type for client requests.
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_message) = match self {
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            AppError::InternalServerError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, message),
        };

        let body = Json(serde_json::json! ({
            "error": err_message,
        }));

        (status, body).into_response()
    }
}
