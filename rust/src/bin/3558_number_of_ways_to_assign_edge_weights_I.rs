impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        if edges.len() == 1 {
            return 1;
        }
        let n = edges.len() + 2;

        let mut adjacency_list = vec![vec![]; n];
        for edge in edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            adjacency_list[u].push(v);
            adjacency_list[v].push(u);
        }

        let mut max_depth = 0;
        let mut stack = Vec::new();
        let mut visted_list = vec![false; n];
        stack.push((1_usize, 0_usize, 0)); // node, visted_index, depth
        visted_list[1] = true;

        while let Some(&(node, edges_index, depth)) = stack.last() {
            max_depth = max_depth.max(depth);
            if edges_index < adjacency_list[node].len() {
                if let Some((_, ei, _)) = stack.last_mut() {
                    *ei += 1;
                }
                let neighbour = adjacency_list[node][edges_index];
                if !visted_list[neighbour] {
                    visted_list[neighbour] = true;
                    stack.push((neighbour, 0, depth + 1));
                }
            } else {
                stack.pop();
            }
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
