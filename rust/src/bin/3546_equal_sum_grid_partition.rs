use std::ops::ControlFlow;

struct Solution {}

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let prefix_list: Vec<u64> = grid
            .iter()
            .map(|row| row.iter().map(|&x| x as u64).sum::<u64>())
            .scan(0_u64, |acc, x| {
                *acc += x;
                Some(*acc)
            })
            .collect();
        let total: u64 = *prefix_list.last().unwrap();

        if total % 2 == 1 {
            return false;
        }

        let half = total / 2;

        if prefix_list.binary_search(&half).is_ok() {
            return true;
        }

        matches!(
            (0..grid[0].len())
                .map(|c| grid.iter().map(|rows| rows[c] as u64).sum::<u64>())
                .try_fold(0_u64, |acc, x| {
                    let sum = acc + x;
                    if sum == half {
                        ControlFlow::Break(true)
                    } else if sum > half {
                        ControlFlow::Break(false)
                    } else {
                        ControlFlow::Continue(sum)
                    }
                }),
            ControlFlow::Break(true)
        )
    }
}

fn main() {
    assert!(Solution::can_partition_grid(vec![vec![1, 4], vec![2, 3]]));
    assert!(!Solution::can_partition_grid(vec![vec![1, 3], vec![2, 4]]));
}
