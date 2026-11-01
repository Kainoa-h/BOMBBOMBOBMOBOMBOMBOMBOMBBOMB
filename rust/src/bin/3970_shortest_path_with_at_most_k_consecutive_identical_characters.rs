use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        let len = n as usize;
        let k = (k as usize).min(len);

        let mut degrees = edges.iter().fold(vec![0_u32; len], |mut degrees, edge| {
            degrees[edge[0] as usize] += 1;
            degrees
        });

        let prefix_partitions = degrees
            .iter()
            .scan(0, |acc, edges| {
                let prefix = *acc;
                *acc += edges;
                Some(prefix)
            })
            .chain(std::iter::once(edges.len() as u32))
            .collect::<Vec<_>>();

        let mut adj_target = vec![0_u32; edges.len()];
        let mut adj_weight = vec![0_i32; edges.len()];

        for edge in &edges {
            let (from, to, weight) = (edge[0] as usize, edge[1] as u32, edge[2]);
            degrees[from] -= 1;
            let idx = (prefix_partitions[from] + degrees[from]) as usize;
            adj_target[idx] = to;
            adj_weight[idx] = weight;
        }

        let b = labels.into_bytes();
        let mut distances = vec![i32::MAX; len * k];
        distances[0..k].fill(0);
        let mut p_queue = BinaryHeap::new();
        p_queue.push(Reverse((0, 0)));

        while let Some(Reverse((weight, node_conseq))) = p_queue.pop() {
            let node_conseq = node_conseq as usize;
            if weight != distances[node_conseq] {
                continue;
            }
            let (node, conseq) = (node_conseq / k, node_conseq % k);
            if node == len - 1 {
                return weight;
            }
            let (start, end) = (prefix_partitions[node], prefix_partitions[node + 1]);
            let curr_label = b[node];
            for e in start..end {
                let e = e as usize;
                let (target, n_weight) = (adj_target[e] as usize, adj_weight[e]);
                let conseq = if b[target] == curr_label { conseq + 1 } else { 0 };
                if conseq == k {
                    continue;
                }
                let tot_weight = weight + n_weight;
                let idx = target * k + conseq;
                if distances[idx] > tot_weight {
                    distances[idx] = tot_weight;
                    p_queue.push(Reverse((tot_weight, idx as u32)));
                }
            }
        }

        -1
    }
}

struct Solution {}
