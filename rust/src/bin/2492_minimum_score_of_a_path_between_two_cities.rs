impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut dsu = DSU::new(n as usize + 1);
        for r in roads {
            let (a, b, d) = (r[0], r[1], r[2]);
            dsu.merge(a as usize, b as usize, d);
        }
        dsu.dsu[1].1
    }
}

struct DSU {
    dsu: Vec<(usize, i32)>,
}

impl DSU {
    fn new(dsu: usize) -> Self {
        Self {
            dsu: (0..dsu).map(|x| (x, i32::MAX)).collect(),
        }
    }

    fn find(&mut self, idx: usize) -> usize {
        let (i, _d) = self.dsu[idx];
        if i == idx {
            return idx;
        }
        self.dsu[idx].0 = self.find(i);
        self.dsu[idx].0
    }

    fn merge(&mut self, a: usize, b: usize, d: i32) {
        let (root_a, root_b) = (self.find(a), self.find(b));
        let (parent, child) = (root_a.min(root_b), root_a.max(root_b));
        let min_dist = d.min(self.dsu[parent].1).min(self.dsu[child].1);
        self.dsu[child] = (parent, min_dist);
        self.dsu[parent].1 = min_dist;
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::min_score(4, vec![vec![1, 2, 2], vec![1, 3, 4], vec![3, 4, 7]]),
        2
    );

    assert_eq!(
        Solution::min_score(
            13,
            vec![
                vec![2, 12, 1891],
                vec![10, 9, 4138],
                vec![11, 3, 2007],
                vec![1, 10, 9390],
                vec![12, 8, 1915],
                vec![6, 2, 1098],
                vec![5, 4, 2795],
                vec![3, 13, 4562],
                vec![9, 7, 9202],
                vec![4, 6, 6752],
                vec![8, 11, 1480],
                vec![7, 5, 9827]
            ]
        ),
        1098
    );

    assert_eq!(
        Solution::min_score(
            7,
            vec![
                vec![1, 3, 1484],
                vec![3, 2, 3876],
                vec![2, 4, 6823],
                vec![6, 7, 579],
                vec![5, 6, 4436],
                vec![4, 5, 8830]
            ]
        ),
        579
    );
}
