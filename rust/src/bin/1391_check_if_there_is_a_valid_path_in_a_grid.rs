struct Solution {}

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let row_size = grid[0].len();
        let flat_size = grid.len() * row_size;
        let mut dsu = DSU::new(flat_size);
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                let flatted_curr = row_idx * row_size + col_idx;
                if Solution::has_top(*cell)
                    && let Some(x) = row_idx
                        .checked_sub(1)
                        .and_then(|i| grid.get(i))
                        .and_then(|r| r.get(col_idx))
                    && Solution::has_bot(*x)
                {
                    let flattened_top = (row_idx - 1) * row_size + col_idx;
                    dsu.merge(flattened_top, flatted_curr);
                }
                if Solution::has_left(*cell)
                    && let Some(x) = col_idx.checked_sub(1).and_then(|i| grid[row_idx].get(i))
                    && Solution::has_right(*x)
                {
                    let flattened_left = row_idx * row_size + col_idx - 1;
                    dsu.merge(flattened_left, flatted_curr);
                }
            }
        }
        dsu.find(flat_size - 1) == 0
    }

    fn has_top(street: i32) -> bool {
        matches!(street, 2 | 5 | 6)
    }

    fn has_right(street: i32) -> bool {
        matches!(street, 1 | 4 | 6)
    }
    fn has_bot(street: i32) -> bool {
        matches!(street, 2 | 3 | 4)
    }
    fn has_left(street: i32) -> bool {
        matches!(street, 1 | 3 | 5)
    }
}

struct DSU {
    dsu: Vec<usize>,
}

impl DSU {
    fn new(len: usize) -> Self {
        DSU {
            dsu: (0..len).collect(),
        }
    }

    fn find(&mut self, idx: usize) -> usize {
        let mut root = self.dsu[idx];
        while root != self.dsu[root] {
            root = self.dsu[root];
        }

        let mut node = idx;
        while node != root {
            let next = self.dsu[node];
            self.dsu[node] = root;
            node = next;
        }

        root
    }

    fn merge(&mut self, a: usize, b: usize) {
        let a_root = self.find(a);
        let b_root = self.find(b);
        if a_root < b_root {
            self.dsu[b_root] = a_root;
        } else {
            self.dsu[a_root] = b_root;
        }
    }
}

fn main() {
    assert!(Solution::has_valid_path(vec![vec![2, 4, 3], vec![6, 5, 2]]));
    assert!(!Solution::has_valid_path(vec![
        vec![1, 2, 1],
        vec![1, 2, 1]
    ]));
    assert!(!Solution::has_valid_path(vec![vec![1, 1, 2],]));
}
