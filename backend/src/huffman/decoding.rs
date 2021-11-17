use crate::huffman::models::{Decoded, Encoded};

use super::models::HuffmanTree;

pub fn decompress_text(encoding: Encoded) -> Decoded {
    Decoded {
        text: reconstruct_text(&encoding.tree, &encoding.encoded_text.as_bytes()),
    }
}

pub fn reconstruct_text(tree: &HuffmanTree, encoded_text: &[u8]) -> String {
    let mut decoded_text = String::new();
    let mut offset = 0;

    while offset < encoded_text.len() {
        let (value, next_char_offset) = next_char_from_tree(&tree, offset, &encoded_text);
        offset = next_char_offset;

        decoded_text.push(value);
    }

    decoded_text
}

pub fn next_char_from_tree(
    tree: &HuffmanTree,
    offset: usize,
    encoded_text: &[u8],
) -> (char, usize) {
    match tree {
        HuffmanTree::Leaf(leaf) => (leaf.value, offset),

        HuffmanTree::Node(node) => {
            match *encoded_text.get(offset).unwrap() as char {
                // '1' is assigned to right edge, check right edge of tree
                '1' => next_char_from_tree(&(*node.right), offset + 1, encoded_text),

                // '0' is assigned to left edge, check left edge of tree
                '0' => next_char_from_tree(&(*node.left), offset + 1, encoded_text),

                _ => {
                    todo!()
                }
            }
        }
    }
}
