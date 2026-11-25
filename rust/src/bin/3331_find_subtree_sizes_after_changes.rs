impl Solution {
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        let mut adj_list = vec![vec![]; parent.len()];
        for (c, &p) in parent.iter().enumerate().skip(1) {
            adj_list[p as usize].push(c);
        }

        let bytes = s.as_bytes();
        let mut subtree_count = vec![1; parent.len()];
        let mut last_seen = [None::<usize>; 26];
        let mut stack = vec![(0_usize, false, None::<usize>)];
        while let Some((node, is_exiting, previous_matching_ansestor)) = stack.pop() {
            let ch = bytes[node];
            let last_seen_idx = (ch - b'a') as usize;
            if is_exiting {
                last_seen[last_seen_idx] = previous_matching_ansestor;
                if node != 0 {
                    let parent = previous_matching_ansestor.unwrap_or(parent[node] as usize);
                    subtree_count[parent] += subtree_count[node];
                }
                continue;
            }

            stack.push((node, true, last_seen[last_seen_idx]));
            last_seen[last_seen_idx] = Some(node);
            for &child in &adj_list[node] {
                stack.push((child, false, None));
            }
        }

        subtree_count
    }
}

struct Solution {}

fn main() {
    // Solution::find_subtree_sizes(vec![-1, 0, 0, 1, 1, 1], "aaaaaa".to_string());
}
