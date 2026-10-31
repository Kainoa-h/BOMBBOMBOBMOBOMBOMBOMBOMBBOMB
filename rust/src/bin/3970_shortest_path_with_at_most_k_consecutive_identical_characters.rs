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
        p_queue.push(Reverse((0, 0, Reverse(0))));

        while let Some(Reverse((weight, conseq, Reverse(node)))) = p_queue.pop() {
            if node == len - 1 {
                return weight;
            }
            if weight != distances[node * k + conseq] {
                continue;
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
                    p_queue.push(Reverse((tot_weight, conseq, Reverse(next))));
                }
            }
        }

        -1
    }
}

struct Solution {}
