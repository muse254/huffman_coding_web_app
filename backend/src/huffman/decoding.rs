use crate::huffman::models::{Decoded, Encoded};

use super::models::HuffmanTree;

/// This function decodes the given encoded text to utf-8 text.
pub fn decompress(encoding: Encoded) -> Decoded {
    let mut decoded_text = Vec::new();
    let mut offset = 0;

    let encoded_text = encoding.encoded_text.as_bytes();
    while offset < encoded_text.len() {
        let (value, next_char_offset) = next_char_from_tree(&encoding.tree, offset, &encoded_text);
        offset = next_char_offset;

        decoded_text.push(value);
    }

    Decoded {
        name: encoding.name,
        text: decoded_text,
    }
}

/// Returns the next character from the tree and the offset of the next character.
pub fn next_char_from_tree(tree: &HuffmanTree, offset: usize, encoded_text: &[u8]) -> (u8, usize) {
    match tree {
        HuffmanTree::Leaf(leaf) => (leaf.value, offset),

        HuffmanTree::Node(node) => {
            match *encoded_text.get(offset).unwrap() as char {
                // '1' is assigned to right edge, check right edge of tree
                '1' => next_char_from_tree(&(*node.right), offset + 1, encoded_text),

                // '0' is assigned to left edge, check left edge of tree
                '0' => next_char_from_tree(&(*node.left), offset + 1, encoded_text),

                // this case is never called
                _ => {
                    unimplemented!();
                }
            }
        }
    }
}
