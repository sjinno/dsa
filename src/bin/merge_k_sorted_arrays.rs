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
