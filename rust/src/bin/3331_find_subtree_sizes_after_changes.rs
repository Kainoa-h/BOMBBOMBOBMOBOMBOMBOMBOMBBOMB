enum State {
    Enter(usize),
    Exit(usize, Option<usize>),
}

impl Solution {
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        let mut adj_list = vec![vec![]; parent.len()];
        for (c, &p) in parent.iter().enumerate().skip(1) {
            adj_list[p as usize].push(c);
        }

        let bytes = s.as_bytes();
        let mut subtree_count = vec![1; parent.len()];
        let mut last_seen = [None::<usize>; 26];
        let mut stack = vec![State::Enter(0)];
        while let Some(state) = stack.pop() {
            match state {
                State::Enter(node) => {
                    let last_seen_idx = (bytes[node] - b'a') as usize;
                    stack.push(State::Exit(node, last_seen[last_seen_idx]));
                    last_seen[last_seen_idx] = Some(node);
                    for &child in &adj_list[node] {
                        stack.push(State::Enter(child));
                    }
                }
                State::Exit(node, prev_ansestor) => {
                    let last_seen_idx = (bytes[node] - b'a') as usize;
                    last_seen[last_seen_idx] = prev_ansestor;
                    if node != 0 {
                        let parent = prev_ansestor.unwrap_or(parent[node] as usize);
                        subtree_count[parent] += subtree_count[node];
                    }
                }
            }
        }

        subtree_count
    }
}

struct Solution {}

fn main() {
    // Solution::find_subtree_sizes(vec![-1, 0, 0, 1, 1, 1], "aaaaaa".to_string());
}
