use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        let k = k as usize;
        let len = n as usize;
        let adj_list = edges.iter().fold(vec![Vec::new(); len], |mut adj, edge| {
            let (from, to, weight) = (edge[0] as usize, edge[1] as usize, edge[2]);
            adj[from].push((to, weight));
            adj
        });

        let b = labels.into_bytes();
        let mut distances = vec![vec![i32::MAX; k + 1]; len];
        distances[0].fill(0);
        let mut p_queue = BinaryHeap::new();
        p_queue.push(Reverse((0, 0, 1)));

        while let Some(Reverse((node, weight, conseq))) = p_queue.pop() {
            for &(next, n_weight) in &adj_list[node] {
                let conseq = if b[next] == b[node] { conseq + 1 } else { 1 };
                if conseq > k {
                    continue;
                }
                let tot_weight = weight + n_weight;
                if distances[next][conseq] > tot_weight {
                    distances[next][conseq] = tot_weight;
                    p_queue.push(Reverse((next, tot_weight, conseq)));
                }
            }
        }

        distances[len - 1]
            .iter()
            .filter(|&&x| x != i32::MAX)
            .min()
            .cloned()
            .unwrap_or(-1)
    }
}

struct Solution {}
