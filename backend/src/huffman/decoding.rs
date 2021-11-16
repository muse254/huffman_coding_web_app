use crate::huffman::models::{Decoded, Encoded};

use super::models::HuffmanTree;

pub fn decompress_text(encoding: Encoded) -> Decoded {
    Decoded {
        text: reconstruct_text(&encoding.tree, &encoding.encoded_text.as_bytes()),
    }
}

pub fn reconstruct_text(tree: &HuffmanTree, encoded_text: &[u8]) -> String {
    let mut decoded_text = Vec::new();
    let mut offset = 0;
    loop {
        let (value, new_offset) = next_char_from_tree(tree, offset, encoded_text);

        decoded_text.push(value);
        if new_offset == encoded_text.len() - 1 {
            break;
        }
        offset = new_offset;
    }

    String::from_iter(decoded_text)
}

pub fn next_char_from_tree(
    tree: &HuffmanTree,
    offset: usize,
    encoded_text: &[u8],
) -> (char, usize) {
    match tree {
        HuffmanTree::Leaf(leaf) => (leaf.value, offset + 1),

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
