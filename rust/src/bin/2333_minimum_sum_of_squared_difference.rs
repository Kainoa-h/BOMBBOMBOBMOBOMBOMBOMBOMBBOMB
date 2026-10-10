use std::collections::BinaryHeap;

impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let mut moves = k1 + k2;
        let mut heap = nums1
            .iter()
            .zip(nums2.iter())
            .map(|(&a, &b)| (a - b).abs())
            .collect::<BinaryHeap<_>>();

        while moves > 0 {
            let mut x = heap.peek_mut().unwrap();
            if *x == 0 {
                break;
            }
            *x -= 1;
            moves -= 1;
        }

        heap.into_iter().map(|x| (x as i64).pow(2)).sum()
    }
}

struct Solution {}
