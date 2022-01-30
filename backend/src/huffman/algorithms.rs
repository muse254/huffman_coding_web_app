//! This module provides the common algorithms that will be used in the cencoding and decoding
//! phases of the program.
//!
//! The algorithms are also backed with tests.
use math::round;
use std::{cmp, cmp::Ordering};

/// This is a binary search implementation that is generic and order agnostic.
/// The list type and sorted list order is up to the caller to specify.
pub fn binary_search<T, C, F: Fn(&T, &C) -> Ordering>(
    sorted_items: &[T],
    target: C,
    cmp: &F,
) -> Option<usize> {
    let mut l = 0;
    let mut r = sorted_items.len() - 1;

    while l <= r {
        let m = round::floor((l + r) as f64 / 2 as f64, 0) as usize;

        match cmp(&sorted_items[m], &target) {
            Ordering::Less => {
                l = m + 1;
            }
            Ordering::Greater => {
                r = m - 1;
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

/// This is a quicksort algorithm that is type generic and order agnostic.
/// The list type and sorting order is up to the caller to specify.
pub fn quick_sort<T, F: Fn(&T, &T) -> Ordering>(items: &mut [T], cmp: &F) {
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
