use std::cmp::Reverse;

struct Solution {}

impl Solution {
    // After sorting desc
    // The idx + 1 tells us at least how many papers have (idx + 1) citations
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable_by_key(|x| Reverse(*x));
        citations
            .iter()
            .enumerate()
            .take_while(|(idx, count)| **count as usize > *idx)
            .count() as i32
    }
}

fn main() {
    assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
    assert_eq!(Solution::h_index(vec![100]), 1);
    assert_eq!(Solution::h_index(vec![100, 200]), 2);
    assert_eq!(Solution::h_index(vec![1, 1, 1]), 1);
    assert_eq!(Solution::h_index(vec![0, 0, 0]), 0);
    assert_eq!(Solution::h_index(vec![0, 0, 2]), 1);
    assert_eq!(Solution::h_index(vec![2, 2, 2]), 2);
    assert_eq!(Solution::h_index(vec![2, 2, 2, 0]), 2);
    assert_eq!(Solution::h_index(vec![1, 7, 9, 4]), 3);
    assert_eq!(Solution::h_index(vec![9, 7, 4, 1]), 3);
    assert_eq!(Solution::h_index(vec![9, 7, 7, 5, 1]), 4);
}
