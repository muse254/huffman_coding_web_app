use crate::huffman::models::{
    Bit, CompressRequest, Encoded, HuffmanCode, HuffmanCodes, HuffmanLeaf, HuffmanNode, HuffmanTree,
};
use std::{cmp, u16};

type TreeHeap = Vec<HuffmanTree>;

fn build_tree(sym_freq: Vec<(char, u16)>) -> HuffmanTree {
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

fn build_node(heap: &mut TreeHeap) {
    // two trees with least frequency
    let a = give_least_freq(heap);
    let b = give_least_freq(heap);

    // put into new node and re-insert into queue
    let node = HuffmanNode {
        freq: a.freq() + b.freq(),
        left: Box::new(a),
        right: Box::new(b),
    };

    heap.push(HuffmanTree::Node(node));
}

fn give_least_freq(heap: &mut TreeHeap) -> HuffmanTree {
    let mut index_least = 0;
    for (index, huffman_tree) in heap.iter().enumerate() {
        if index != 0 && huffman_tree.freq() < heap[index_least].freq() {
            index_least = index
        }
    }

    heap.remove(index_least)
}

fn build_huffman_codes(huffman_tree: &HuffmanTree, code: Vec<Bit>, codes: &mut HuffmanCodes) {
    match huffman_tree {
        HuffmanTree::Leaf(leaf) => {
            codes.huffman_codes.push(HuffmanCode {
                character: leaf.value.into(),
                frequency: leaf.freq,
                huffman_code: code,
            });
        }

        HuffmanTree::Node(node) => {
            let mut mut_code = code;

            // Assign 0 to left edge and recur
            mut_code.push(Bit::Zero);
            build_huffman_codes(&node.left, mut_code.clone(), codes);
            mut_code.pop();

            // Assign 1 to right edge and recur
            mut_code.push(Bit::One);
            build_huffman_codes(&node.right, mut_code, codes);
        }
    }
}

pub fn compress_text(compress_req: CompressRequest) -> Encoded {
    match compress_req {
        CompressRequest::Text(text) => {
            // build hash-map from text
            let mut sym_freq: Vec<(char, u16)> = Vec::new();

            for c in text.chars() {
                let mut exists = false;

                // if key exists update, if not add as new entry
                for (key, value) in &mut sym_freq {
                    if *key == c {
                        *value += 1;
                        exists = true;
                    }
                }
                if !exists {
                    sym_freq.push((c, 1))
                }
            }

            // create a priority queue
            quick_sort(&mut sym_freq);

            // build the Huffman tree
            let huffman_tree = build_tree(sym_freq);

            // from the Huffman tree, obtain the Huffman codes
            let mut huffman_codes = HuffmanCodes::new();
            build_huffman_codes(&huffman_tree, Vec::new(), &mut huffman_codes);

            Encoded {
                tree: huffman_tree,
                codes: huffman_codes,
                encoded_text: Vec::new(),
            }
        }

        CompressRequest::File => todo!(),
    }
}

fn quick_sort(sym_freq: &mut [(char, u16)]) {
    let len = sym_freq.len();
    if len <= 1 {
        return;
    }

    let pivot = 0;
    sym_freq.swap(pivot, len / 2);

    let mut left = 1;
    let mut right = sym_freq.len() - 1;

    loop {
        while left < len && {
            let (_, val1) = &sym_freq[left];
            let (_, val2) = &sym_freq[pivot];
            val1 > val2
        } {
            left += 1
        }
        while right > 0 && {
            let (_, val1) = &sym_freq[right];
            let (_, val2) = &sym_freq[pivot];
            val1 < val2
        } {
            right -= 1
        }
        if left >= right {
            break;
        }

        sym_freq.swap(left, right);
        left += 1;
        right -= 1;
    }

    sym_freq.swap(pivot, right);
    quick_sort(&mut sym_freq[0..cmp::min(left - 1, right)]);
    quick_sort(&mut sym_freq[cmp::max(left, right + 1)..]);
}

#[test]
fn quick_sort_test() {
    let mut vec_to_sort = vec![('1', 54), ('1', 65), ('1', 3), ('1', 68)];
    let vec_to_expect = vec![('1', 68), ('1', 65), ('1', 54), ('1', 3)];

    quick_sort(&mut vec_to_sort);
    assert_eq!(vec_to_sort, vec_to_expect);
}
