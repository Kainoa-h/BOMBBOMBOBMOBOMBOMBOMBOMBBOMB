use std::collections::HashMap;

impl Solution {
    pub fn assign_edge_weights(mut edges: Vec<Vec<i32>>) -> i32 {
        if edges.len() == 1 {
            return 1;
        }
        let mut max_depth = 0;
        let mut map = HashMap::new();
        edges.sort_unstable_by_key(|x| x[0]);
        for edge_pair in edges {
            let (u, v) = (edge_pair[0], edge_pair[1]);
            let (parent, child) = (u.min(v), u.max(v));
            let depth = *map.entry(parent).or_insert(0) + 1;
            map.insert(child, depth);
            max_depth = max_depth.max(depth);
        }

        let mut ans: i64 = 1;
        let mut base: i64 = 2;
        let mut exp = max_depth - 1;
        let modulo: i64 = 1_000_000_007;

        while exp > 0 {
            if exp % 2 == 1 {
                ans = (ans * base) % modulo;
            }
            base = (base * base) % modulo;
            exp /= 2;
        }

        ans as i32
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::assign_edge_weights(vec![vec![1, 2]]), 1);
    assert_eq!(
        Solution::assign_edge_weights(vec![vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]]),
        2
    );
    assert_eq!(
        Solution::assign_edge_weights(vec![vec![2, 3], vec![1, 2]]),
        2
    );
    assert_eq!(
        Solution::assign_edge_weights(vec![vec![1, 5], vec![5, 2], vec![5, 3], vec![5, 4]]),
        2
    );
}
