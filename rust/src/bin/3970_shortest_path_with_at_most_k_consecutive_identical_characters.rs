use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        let k = k as usize;
        let len = n as usize;
        let mut adj_head = vec![None::<usize>; len];
        let mut adj_next = vec![None::<usize>; edges.len()];
        let mut adj_target = vec![0_usize; edges.len()];
        let mut adj_weight = vec![0_i32; edges.len()];

        for (e_idx, edge) in edges.iter().enumerate() {
            let (from, to, weight) = (edge[0] as usize, edge[1] as usize, edge[2]);
            adj_target[e_idx] = to;
            adj_weight[e_idx] = weight;

            adj_next[e_idx] = adj_head[from];
            adj_head[from] = Some(e_idx);
        }

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
            let mut e_idx = adj_head[node];
            while let Some(e) = e_idx {
                e_idx = adj_next[e];
                let (target, n_weight) = (adj_target[e], adj_weight[e]);
                let conseq = if b[target] == b[node] { conseq + 1 } else { 0 };
                if conseq == k {
                    continue;
                }
                let tot_weight = weight + n_weight;
                let idx = target * k + conseq;
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
