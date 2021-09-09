use crate::models::{HuffmanCode, HuffmanCodes};
use std::{collections::HashMap, usize};

enum HuffmanTree {
    Leaf(HuffmanLeaf),
    Node(HuffmanNode),
}

impl HuffmanTree {
    fn freq(&self) -> u32 {
        match self {
            HuffmanTree::Leaf(leaf) => leaf.freq,
            HuffmanTree::Node(node) => node.freq,
        }
    }
}

struct HuffmanLeaf {
    freq: u32,
    value: char,
}

struct HuffmanNode {
    freq: u32,
    left: Box<HuffmanTree>,
    right: Box<HuffmanTree>,
}

type TreeHeap = Vec<HuffmanTree>;

trait Sort {
    fn len(&self) -> u32;
    fn swap_elem(&mut self, i: &u32, j: &u32);
    fn less(&self, i: &u32, j: &u32) -> bool;
}

impl Sort for TreeHeap {
    fn len(&self) -> u32 {
        self.len() as u32
    }

    fn swap_elem(&mut self, i: &u32, j: &u32) {
        self.swap(*i as usize, *j as usize)
    }

    fn less(&self, i: &u32, j: &u32) -> bool {
        let val_i = self.get(*i as usize).unwrap();
        let val_j = self.get(*j as usize).unwrap();
        val_i.freq() < val_j.freq()
    }
}

fn heapify(h: &mut TreeHeap) {
    let n = h.len() as i32;

    let mut i = (n / 2) - 1;
    while i >= 0 {
        down(h, i as u32, n as u32);
        i -= 1;
    }
}

fn down(tree: &mut TreeHeap, i0: u32, n: u32) {
    let mut i = i0 as i32;
    loop {
        let j1 = 2 * i + 1;
        if j1 >= n as i32 || j1 < 0 {
            // j1 < 0 after int overflow
            break;
        }

        let mut j = j1; // left child
        let j2 = j1 + 1; // = 2*i + 2
        if j2 < n as i32 && tree.less(&(j2 as u32), &(j1 as u32)) {
            j = j2; // right child
        }

        if !tree.less(&(j as u32), &(i as u32)) {
            break;
        }

        tree.swap_elem(&(i as u32), &(j as u32));
        i = j;
    }
}

fn up(h: &mut TreeHeap, j: &mut u32) {
    loop {
        let i = (*j - 1) / 2; // parent
        if i == *j || !h.less(j, &i) {
            break;
        }
        h.swap_elem(&i, j);
        *j = i;
    }
}

// pop removes and returns the minimum element (according to less) from the heap.
// The complexity is O(log n) where n = h.Len().
fn pop<'a>(h: &mut TreeHeap) -> (HuffmanTree, &mut TreeHeap) {
    let n = h.len();
    h.swap_elem(&0, &(n as u32));
    down(h, 0, n as u32);

    (h.pop().unwrap(), h)
}

// push pushes the element x onto the heap.
// The complexity is O(log n) where n = h.Len().
fn push<'a>(h: &mut TreeHeap, x: HuffmanTree) {
    h.push(x);
    let mut y = (h.len() - 1) as u32;
    up(h, &mut y);
}

fn build_tree(sym_freq: &HashMap<char, u32>) -> HuffmanTree {
    let mut trees = TreeHeap::new();

    for (c, f) in sym_freq {
        trees.push(HuffmanTree::Leaf(HuffmanLeaf {
            freq: *f,
            value: *c,
        }))
    }

    let trees_ref = &mut trees;
    heapify(trees_ref);
    let mut len = trees.len();

    while len > 1 {
        let trees_ref = &mut trees;
        build_node(trees_ref);

        len = trees_ref.len();
    }

    let trees_ref = &mut trees;
    let (tree, _) = pop(trees_ref);
    tree
}

fn build_node<'a>(trees: &mut TreeHeap) {
    // two trees with least frequency
    let (a, trees_ref_1) = pop(trees);
    let (b, trees_ref_2) = pop(trees_ref_1);

    // put into new node and re-insert into queue
    let node = HuffmanNode {
        freq: a.freq() + b.freq(),
        left: Box::new(a),
        right: Box::new(b),
    };

    push(trees_ref_2, HuffmanTree::Node(node));
}

fn build_huffman_codes(
    huffman_tree: &HuffmanTree,
    code: Vec<u8>,
    huffman_codes: &mut HuffmanCodes,
) -> Vec<u8> {
    match huffman_tree {
        // If this is a leaf node, then it contains one of the input
        // characters
        HuffmanTree::Leaf(leaf) => {
            let cloned_code = code.clone();

            let huffman_code = HuffmanCode {
                character: leaf.value,
                frequency: leaf.freq,
                huffman_code: code,
            };

            huffman_codes.huffman_codes.push(Some(huffman_code));
            cloned_code
        }

        HuffmanTree::Node(node) => {
            let mut mut_code = code;
            let huffman_codes_ref = &mut (*huffman_codes);

            // Assign 0 to left edge and recur
            mut_code.push('0' as u8);
            mut_code = build_huffman_codes(node.left.as_ref(), mut_code, huffman_codes_ref);
            let index = huffman_codes_ref.huffman_codes.len() - 1;
            huffman_codes_ref.huffman_codes.remove(index);

            // Assign 1 to right edge and recur
            mut_code.push('1' as u8);
            mut_code = build_huffman_codes(node.right.as_ref(), mut_code, huffman_codes_ref);
            huffman_codes
                .huffman_codes
                .remove(huffman_codes.huffman_codes.len() - 1);

            mut_code
        }
    }
}

pub fn generate_codes<'a>(text: &'a str) -> HuffmanCodes {
    // build hash-map from string
    let mut sym_freq: HashMap<char, u32> = HashMap::new();

    for c in text.chars() {
        // populate the hashmap
        // update a key, guarding against the key possibly not being set
        let sym_freq_ref = sym_freq.entry(c).or_insert(0);
        *sym_freq_ref += 1;
    }

    let huffman_tree = build_tree(&sym_freq);
    let mut huffman_codes = HuffmanCodes::new();

    {
        let huffman_codes_ref = &mut huffman_codes;
        build_huffman_codes(&huffman_tree, Vec::new(), huffman_codes_ref);
    }

    huffman_codes
}
