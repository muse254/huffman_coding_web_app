//! This module handles all encoding of the Huffman Coding algorithm.

use crate::huffman::algorithms::{binary_search, quick_sort};
use crate::huffman::models::{
    Encoded, HuffmanCode, HuffmanCodes, HuffmanLeaf, HuffmanNode, HuffmanTree, TextFile,
};
use std::u16;

type TreeHeap = Vec<HuffmanTree>;

/// This function encodes the given chracter frequency map to a Huffman Tree.
fn build_tree(sym_freq: Vec<(u8, u16)>) -> HuffmanTree {
    let mut trees = TreeHeap::new();
    for (c, f) in sym_freq {
        trees.push(HuffmanTree::Leaf(HuffmanLeaf { freq: f, value: c }));
    }

    let trees_ref = &mut trees;
    while trees_ref.len() > 1 {
        build_node(trees_ref);
    }

    trees_ref.pop().unwrap()
}

/// Builds a HuffmanNode from two HuffmanLeafs.
fn build_node(heap: &mut TreeHeap) {
    // two trees with least frequency
    let a = give_least_freq(heap);
    let b = give_least_freq(heap);

    // put into new node and re-insert into queue
    let node = HuffmanNode {
        freq: a.freq() + b.freq(),
        left: Box::new(b),
        right: Box::new(a),
    };

    heap.push(HuffmanTree::Node(node));
}

/// Returns the tree with the least frequency.
fn give_least_freq(heap: &mut TreeHeap) -> HuffmanTree {
    let mut index_least = 0;
    for (index, huffman_tree) in heap.iter().enumerate() {
        if index != 0 && huffman_tree.freq() <= heap[index_least].freq() {
            index_least = index
        }
    }

    heap.remove(index_least)
}

/// Builds the Huffman codes for the given tree.
fn build_huffman_codes(huffman_tree: &HuffmanTree, code: Vec<u8>, codes: &mut HuffmanCodes) {
    match huffman_tree {
        HuffmanTree::Leaf(leaf) => {
            codes.huffman_codes.push(HuffmanCode {
                character: leaf.value,
                frequency: leaf.freq,
                huffman_code: String::from_utf8(code).unwrap(),
            });
        }

        HuffmanTree::Node(node) => {
            let mut mut_code = code;

            // Assign 0 to left edge and recur
            mut_code.push('0' as u8);
            build_huffman_codes(&node.left, mut_code.clone(), codes);
            mut_code.pop();

            // Assign 1 to right edge and recur
            mut_code.push('1' as u8);
            build_huffman_codes(&node.right, mut_code, codes);
        }
    }
}

/// Populates the sym_freq vector with the character frequency of each character in the data set.
fn update_occurences(data_set: &[u8]) -> Vec<(u8, u16)> {
    let mut sym_freq: Vec<(u8, u16)> = Vec::new();
    for c in data_set {
        let mut exists = false;

        // if key exists update, if not add as new entry
        for (key, value) in &mut sym_freq {
            if *key == *c {
                *value += 1;
                exists = true;
            }
        }
        if !exists {
            sym_freq.push((c.clone(), 1))
        }
    }
    sym_freq
}

/// Encodes the text provided to a Huffman Tree and Huffman Codes.
pub fn compress<'a>(text: TextFile) -> Encoded {
    // build hash-map from text
    let mut sym_freq: Vec<(u8, u16)> = update_occurences(&text.file);

    // create a priority queue
    quick_sort(&mut sym_freq, &|x, y| x.1.cmp(&y.1));

    // build the Huffman tree
    let huffman_tree = build_tree(sym_freq);

    // from the Huffman tree, obtain the Huffman codes
    let mut huffman_codes = HuffmanCodes::new();
    build_huffman_codes(&huffman_tree, Vec::new(), &mut huffman_codes);

    // sort the huffman codes according to their char value in ascending order
    quick_sort(&mut huffman_codes.huffman_codes, &|x, y| {
        y.character.cmp(&x.character)
    });

    let encoded_text = generate_encoded_text(&huffman_codes, &text.file);

    Encoded {
        name: text.alpha,
        tree: huffman_tree,
        codes: huffman_codes,
        encoded_text: encoded_text,
    }
}

/// Generates the encoded text from the given Huffman codes.
fn generate_encoded_text<'a>(codes: &HuffmanCodes, text: &[u8]) -> String {
    let mut encoded_text = String::new();
    for c in text {
        let index =
            binary_search(&codes.huffman_codes, c, &|codes, c| codes.character.cmp(&c)).unwrap();

        // update the encoded text
        let target = codes.huffman_codes.get(index).unwrap();
        encoded_text.push_str(&target.huffman_code);
    }

    encoded_text
}
