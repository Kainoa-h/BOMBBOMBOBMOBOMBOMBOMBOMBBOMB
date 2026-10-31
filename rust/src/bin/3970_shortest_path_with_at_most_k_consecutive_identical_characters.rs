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
        let mut distances = vec![i32::MAX; len * k];
        distances[0..k].fill(0);
        let mut p_queue = BinaryHeap::new();
        p_queue.push(Reverse((0, 0)));

        while let Some(Reverse((weight, node_conseq))) = p_queue.pop() {
            if weight != distances[node_conseq] {
                continue;
            }
            let (node, conseq) = (node_conseq / k, node_conseq % k);
            if node == len - 1 {
                return weight;
            }
            for &(next, n_weight) in &adj_list[node] {
                let conseq = if b[next] == b[node] { conseq + 1 } else { 0 };
                if conseq == k {
                    continue;
                }
                let tot_weight = weight + n_weight;
                let idx = next * k + conseq;
                if distances[idx] > tot_weight {
                    distances[idx] = tot_weight;
                    p_queue.push(Reverse((tot_weight, idx)));
                }
            }
        }

        -1
    }
}

struct Solution {}
