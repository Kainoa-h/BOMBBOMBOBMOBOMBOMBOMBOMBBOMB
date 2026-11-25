impl Solution {
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        let mut adj_list = vec![vec![]; parent.len()];
        for (c, &p) in parent.iter().enumerate().skip(1) {
            adj_list[p as usize].push(c);
        }

        let bytes = s.as_bytes();
        let mut new_parent = parent.clone();
        let mut last_seen = [None::<usize>; 26];
        let mut stack = vec![(0_usize, false, None::<usize>)];
        while let Some((node, is_exiting, previous_matching_ansestor)) = stack.pop() {
            let ch = bytes[node];
            let last_seen_idx = (ch - b'a') as usize;
            if is_exiting {
                last_seen[last_seen_idx] = previous_matching_ansestor;
                continue;
            }

            stack.push((node, true, last_seen[last_seen_idx]));
            if let Some(p) = last_seen[last_seen_idx] {
                new_parent[node] = p as i32;
            }
            last_seen[last_seen_idx] = Some(node);
            for &child in &adj_list[node] {
                stack.push((child, false, None));
            }
        }

        let mut adj_list = vec![vec![]; parent.len()];
        for (c, &p) in new_parent.iter().enumerate().skip(1) {
            adj_list[p as usize].push(c);
        }
        let mut stack = vec![(0_usize, false)];
        let mut subtree_count = vec![0; parent.len()];
        while let Some((node, is_exiting)) = stack.pop() {
            if is_exiting {
                subtree_count[node] = 1 + adj_list[node]
                    .iter().map(|&c| subtree_count[c]).sum::<i32>();
                continue;
            }
            
            if adj_list[node].is_empty() {
                subtree_count[node] = 1;
                continue;
            }
            
            stack.push((node, true));
            for &child in &adj_list[node] {
                stack.push((child, false));
            }
        }

        subtree_count
    }
}

struct Solution {}

fn main() {
    // Solution::find_subtree_sizes(vec![-1, 0, 0, 1, 1, 1], "aaaaaa".to_string());
}
