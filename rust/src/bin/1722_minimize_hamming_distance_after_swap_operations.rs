use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        let mut diff = 0;
        let mut dsu = DSU::new(source.len());
        let mut root_to_values: HashMap<usize, HashMap<i32, i32>> = HashMap::new();

        for swap in allowed_swaps {
            dsu.merge(swap[0] as usize, swap[1] as usize);
        }

        for (idx, val) in source.iter().enumerate() {
            root_to_values
                .entry(dsu.find(idx))
                .or_default()
                .entry(*val)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        for (idx, val) in target.iter().enumerate() {
            let values: &mut HashMap<i32, i32> = root_to_values.get_mut(&dsu.find(idx)).unwrap();
            if values.get(val).is_some_and(|&v| v > 0) {
                values.entry(*val).and_modify(|v| *v -= 1);
            } else {
                diff += 1;
            }
        }

        diff
    }
}

struct DSU {
    dsu: Vec<usize>,
}

impl DSU {
    pub fn new(size: usize) -> Self {
        Self {
            dsu: (0..size).collect(),
        }
    }
    pub fn find(&mut self, idx: usize) -> usize {
        if self.dsu[idx] == idx {
            return idx;
        }
        self.dsu[idx] = self.find(self.dsu[idx]);
        self.dsu[idx]
    }
    pub fn merge(&mut self, a: usize, b: usize) {
        let root_b = self.find(b);
        let root_a = self.find(a);
        self.dsu[root_b] = root_a;
    }
}

fn main() {
    assert_eq!(
        Solution::minimum_hamming_distance(
            vec![1, 2, 3, 4],
            vec![2, 1, 4, 5],
            vec![vec![0, 1], vec![2, 3]],
        ),
        1
    );

    assert_eq!(
        Solution::minimum_hamming_distance(vec![1, 2, 3, 4], vec![1, 3, 2, 4], vec![],),
        2
    );

    assert_eq!(
        Solution::minimum_hamming_distance(
            vec![5, 1, 2, 4, 3],
            vec![1, 5, 4, 2, 3],
            vec![vec![0, 4], vec![4, 2], vec![1, 3], vec![1, 4]],
        ),
        0
    );
}
