//! This module containing all the models used in the decoding and decoding processes.
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Encoded {
    pub name: Option<String>,
    pub codes: HuffmanCodes,
    pub tree: HuffmanTree,
    pub encoded_text: String,
}

#[derive(Debug, Serialize)]
pub struct Decoded {
    pub name: Option<String>,
    pub text: Vec<u8>,
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

impl HuffmanTree {
    /// This checks whether the tree contains only a single node(leaf).
    pub fn is_single_node(&self) -> bool {
        match self {
            HuffmanTree::Leaf(_) => true,
            HuffmanTree::Node(_) => false,
        }
    }

    /// This retrieves the value of the single node in the tree.
    pub fn get_single_node_value(&self) -> u8 {
        match self {
            HuffmanTree::Node(_) => panic!("This is not a single node tree"),
            HuffmanTree::Leaf(leaf) => leaf.value,
        }
    }

    pub fn get_single_node_frequency(&self) -> u16 {
        match self {
            HuffmanTree::Node(_) => panic!("This is not a single node tree"),
            HuffmanTree::Leaf(leaf) => leaf.freq,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct HuffmanLeaf {
    pub freq: u16,
    pub value: u8,
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
    pub character: u8,
    pub frequency: u16,
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
