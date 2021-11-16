use crate::huffman::models::{
    Encoded, HuffmanCode, HuffmanCodes, HuffmanLeaf, HuffmanNode, HuffmanTree,
};
use math::round;
use std::{cmp, cmp::Ordering, str, u16};

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

fn build_huffman_codes(huffman_tree: &HuffmanTree, code: Vec<u8>, codes: &mut HuffmanCodes) {
    match huffman_tree {
        HuffmanTree::Leaf(leaf) => {
            codes.huffman_codes.push(HuffmanCode {
                character: leaf.value.into(),
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

pub fn compress_text<'a>(text: &'a str) -> Encoded {
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

    let encoded_text = generate_encoded_text(&huffman_codes, text);

    Encoded {
        tree: huffman_tree,
        codes: huffman_codes,
        encoded_text: encoded_text,
    }
}

fn generate_encoded_text<'a>(codes: &HuffmanCodes, text: &'a str) -> String {
    let mut encoded_text = String::new();
    for c in text.chars() {
        let index =
            binary_search(&codes.huffman_codes, c, &|codes, c| codes.character.cmp(&c)).unwrap();

        // update the encoded text
        let target = codes.huffman_codes.get(index).unwrap();
        encoded_text.push_str(&target.huffman_code);
    }

    encoded_text
}

fn binary_search<T, C, F>(sorted_items: &[T], target: C, cmp: &F) -> Option<usize>
where
    F: Fn(&T, &C) -> Ordering,
{
    let mut l = 0;
    let mut r = (sorted_items.len() - 1) as i32;

    // the floor function is the function that takes as input a real number x,
    // and gives as output the greatest integer less than or equal to x
    let floor = |dividend, divisor| round::floor(dividend / divisor, 0) as usize;

    while l <= r {
        let m = floor((l + r) as f64, 2 as f64);

        match cmp(&sorted_items[m], &target) {
            Ordering::Less => {
                l = m as i32 + 1;
            }
            Ordering::Greater => {
                r = m as i32 - 1;
            }
            Ordering::Equal => return Some(m),
        }
    }

    None
}

#[test]
fn binary_search_test() {
    let mut example = vec![('1', 3), ('1', 54), ('1', 65), ('1', 68), ('1', 69)];

    // ascending order
    let mut asc_res = binary_search(&example, 3, &|x, y| x.1.cmp(&y));
    assert_eq!(asc_res, Some(0));

    asc_res = binary_search(&example, 68, &|x, y| x.1.cmp(&y));
    assert_eq!(asc_res, Some(3));

    // descending order
    example.reverse();
    let desc_res = binary_search(&example, 3, &|x, y| y.cmp(&x.1));
    assert_eq!(desc_res, Some(4));
}

fn quick_sort<T, F>(items: &mut [T], cmp: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let len = items.len();
    if len <= 1 {
        return;
    }

    let pivot = 0;
    items.swap(pivot, len / 2);

    let mut left = 1;
    let mut right = items.len() - 1;

    loop {
        while left < len && cmp(&items[left], &items[pivot]) == Ordering::Greater {
            left += 1
        }
        while right > 0 && cmp(&items[right], &items[pivot]) == Ordering::Less {
            right -= 1
        }
        if left >= right {
            break;
        }

        items.swap(left, right);
        left += 1;
        right -= 1;
    }

    items.swap(pivot, right);
    quick_sort(&mut items[0..cmp::min(left - 1, right)], cmp);
    quick_sort(&mut items[cmp::max(left, right + 1)..], cmp);
}

#[test]
fn quick_sort_test() {
    let mut vec_to_sort = vec![('1', 54), ('1', 65), ('1', 3), ('1', 68), ('1', 69)];
    let mut vec_to_expect = vec![('1', 69), ('1', 68), ('1', 65), ('1', 54), ('1', 3)];

    // descending order
    quick_sort(&mut vec_to_sort, &|x, y| x.1.cmp(&y.1));
    assert_eq!(vec_to_sort, vec_to_expect);

    // ascending order
    quick_sort(&mut vec_to_sort, &|x, y| y.1.cmp(&x.1));
    vec_to_expect.reverse();
    assert_eq!(vec_to_sort, vec_to_expect);
}
