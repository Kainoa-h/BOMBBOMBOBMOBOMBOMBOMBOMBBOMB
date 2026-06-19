struct DSU {
    dsu: Vec<usize>,
}

impl DSU {
    fn new(size: usize) -> Self {
        DSU {
            dsu: (0..size).collect(),
        }
    }

    fn find(&mut self, index: usize) -> usize {
        if self.dsu[index] != index {
            self.dsu[index] = self.find(self.dsu[index]);
        }
        self.dsu[index]
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        self.dsu[y_root] = x_root;
    }
}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut dsu = DSU::new(intervals.len());

        for i in 0..intervals.len() {
            let (x1, x2) = (intervals[i][0], intervals[i][1]);
            for j in i..intervals.len() {
                let (y1, y2) = (intervals[j][0], intervals[j][1]);
                if (x1 >= y1 && x1 <= y2)
                    || (x2 >= y1 && x2 <= y2)
                    || (y1 >= x1 && y1 <= x2)
                    || (y2 >= x1 && y2 <= x2)
                {
                    dsu.union(i, j);
                }
            }
        }

        let mut map = vec![None::<Vec<i32>>; intervals.len()];
        for (i, inter) in intervals.iter().enumerate() {
            let root = dsu.find(i);
            let combi_interval = map[root].get_or_insert(vec![inter[0], inter[1]]);
            combi_interval[0] = combi_interval[0].min(inter[0]);
            combi_interval[1] = combi_interval[1].max(inter[1]);
        }

        let mut result: Vec<Vec<i32>> = map.into_iter().flatten().collect();
        result.sort_unstable_by_key(|x| x[0]);
        result
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
        vec![vec![1, 6], vec![8, 10], vec![15, 18]]
    )
}
/*
Example 1:

Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
Output: [[1,6],[8,10],[15,18]]
Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
Example 2:

Input: intervals = [[1,4],[4,5]]
Output: [[1,5]]
Explanation: Intervals [1,4] and [4,5] are considered overlapping.
Example 3:

Input: intervals = [[4,7],[1,4]]
Output: [[1,7]]
Explanation: Intervals [1,4] and [4,7] are considered overlapping.
*/
