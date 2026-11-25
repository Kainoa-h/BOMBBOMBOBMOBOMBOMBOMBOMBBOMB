impl Solution {
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        let mut adj_list = vec![Vec::<usize>::new(); s.len()];
        for (c, &p) in parent.iter().enumerate().skip(1) {
            adj_list[p as usize].push(c);
        }

        let letters = s.as_bytes();
        let mut matching_parent = [None::<usize>; 26];
        let mut stack = vec![(0, 0)];
        while let Some(&(node, parent)) = stack.last() {
            let letter_idx = (letters[node] - b'a') as usize;
            if let Some(matching) = matching_parent[letter_idx] {
                if matching == node {
                    matching_parent[letter_idx] = None;
                } else {
                    for &child in &adj_list[node] {
                        stack.push((child, parent));
                    }
                    let idx = adj_list[parent].iter().position(|&x| x == node).unwrap();
                    adj_list[parent].remove(idx);
                    let childrens = adj_list[node].clone();
                    adj_list[parent].extend(childrens);
                    adj_list[node].clear();
                    adj_list[matching].push(node);
                }
                let _ = stack.pop();
            } else {
                matching_parent[letter_idx] = Some(node);
                for &child in &adj_list[node] {
                    stack.push((child, node));
                }
            }
        }

        let mut subtree_count = vec![0_i32; s.len()];
        let mut stack = vec![0_usize];
        while let Some(&top) = stack.last() {
            if adj_list[top].is_empty() {
                subtree_count[top] = 1;
                let _ = stack.pop();
                continue;
            }
            if adj_list[top].iter().any(|&c| subtree_count[c] > 0) {
                subtree_count[top] =
                    adj_list[top].iter().map(|&c| subtree_count[c]).sum::<i32>() + 1_i32;
                let _ = stack.pop();
                continue;
            }
            for &c in &adj_list[top] {
                stack.push(c);
            }
        }
        subtree_count
    }
}

struct Solution {}

fn main() {
    // Solution::find_subtree_sizes(vec![-1, 0, 0, 1, 1, 1], "aaaaaa".to_string());
}
