struct Solution {}

#[derive(Clone, Copy, Default)]
struct Node {
    sum: i32,
    has_x: bool,
}

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let cols = grid[0].len();
        let mut prefix_row = vec![Node::default(); cols];
        for row in grid {
            let mut row_sum = 0;
            let mut row_has_x = false;
            for (c_idx, node) in prefix_row.iter_mut().enumerate() {
                row_sum += match row[c_idx] {
                    'X' => 1,
                    'Y' => -1,
                    _ => 0,
                };
                if row[c_idx] == 'X' {
                    row_has_x = true;
                }

                let sum = node.sum + row_sum;
                let has_x = node.has_x || row_has_x;
                *node = Node { sum, has_x };

                if sum == 0 && has_x {
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    assert_eq!(
        3,
        Solution::number_of_submatrices(vec![vec!['X', 'Y', '.'], vec!['Y', '.', '.']])
    )
}
