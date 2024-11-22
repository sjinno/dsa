use std::{cmp::Reverse, collections::BinaryHeap};

fn merge_k_sorted_arrays(arrays: Vec<Vec<i32>>) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    let mut result = Vec::new();

    for (i, array) in arrays.iter().enumerate() {
        heap.push(Reverse((array[0], i, 0)));
    }

    while let Some(Reverse((value, i, j))) = heap.pop() {
        result.push(value);
        if j + 1 < arrays[i].len() {
            heap.push(Reverse((arrays[i][j + 1], i, j + 1)));
        }
    }

    result
}

#[test]
fn test_merge_k_sorted_arrays() {
    let arrays = vec![vec![1, 4, 7, 15], vec![2, 5, 7, 8], vec![3, 6, 9, 12, 15]];
    assert_eq!(
        merge_k_sorted_arrays(arrays),
        vec![1, 2, 3, 4, 5, 6, 7, 7, 8, 9, 12, 15, 15]
    );

    assert_eq!(
        merge_k_sorted_arrays(vec![vec![1, 3], vec![2, 4], vec![5, 6]]),
        vec![1, 2, 3, 4, 5, 6],
    );
}

fn merge_k_sorted_arrays2(arrays: Vec<Vec<i32>>) -> Vec<i32> {
    if arrays.is_empty() {
        return vec![];
    }

    merge_sort(&arrays, 0, arrays.len() - 1)
}

fn merge_sort(arrays: &[Vec<i32>], left: usize, right: usize) -> Vec<i32> {
    if left == right {
        return arrays[left].clone();
    }

    let mid = left + (right - left) / 2;
    let left = merge_sort(arrays, left, mid);
    let right = merge_sort(arrays, mid + 1, right);

    merge_two_sorted_arrays(left, right)
}

fn merge_two_sorted_arrays(sorted_array1: Vec<i32>, sorted_array2: Vec<i32>) -> Vec<i32> {
    let len1 = sorted_array1.len();
    let len2 = sorted_array2.len();

    let mut p1 = 0;
    let mut p2 = 0;

    let mut merged_array = Vec::<i32>::with_capacity(len1 + len2);

    while p1 < len1 && p2 < len2 {
        if sorted_array1[p1] < sorted_array2[p2] {
            merged_array.push(sorted_array1[p1]);
            p1 += 1;
        } else {
            merged_array.push(sorted_array2[p2]);
            p2 += 1;
        }
    }

    // Push whatever's left to merged_array if any
    merged_array.extend_from_slice(&sorted_array1[p1..]);
    merged_array.extend_from_slice(&sorted_array2[p2..]);

    merged_array
}

#[test]
fn test_merge_two_sorted_arrays() {
    let a1 = vec![1, 2];
    let a2 = vec![2, 3, 4];
    assert_eq!(merge_two_sorted_arrays(a1, a2), vec![1, 2, 2, 3, 4]);
}

#[test]
fn test_merge_k_sorted_arrays2() {
    let arrays = vec![vec![1, 4, 7, 15], vec![2, 5, 7, 8], vec![3, 6, 9, 12, 15]];
    assert_eq!(
        merge_k_sorted_arrays2(arrays),
        vec![1, 2, 3, 4, 5, 6, 7, 7, 8, 9, 12, 15, 15]
    );

    assert_eq!(
        merge_k_sorted_arrays2(vec![vec![1, 3], vec![2, 4], vec![5, 6]]),
        vec![1, 2, 3, 4, 5, 6],
    );
}
