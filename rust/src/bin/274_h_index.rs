use std::cmp::Reverse;

struct Solution {}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations.clone();
        citations.sort_by_key(|x| Reverse(*x));
        let mut seen = 0;
        for &c in &citations {
            if c < 1 {
                break;
            }
            seen += 1;
            if c > seen && citations.get(seen as usize).copied().unwrap_or(0) < seen {
                return seen;
            }
            if c <= seen {
                return c;
            }
        }
        seen
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
