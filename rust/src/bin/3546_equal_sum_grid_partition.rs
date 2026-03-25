struct Solution {}

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let total: u64 = grid.iter().flatten().map(|&x| x as u64).sum();

        if total % 2 == 1 {
            return false;
        }

        let half = total / 2;

        let mut sum = 0;
        for row in &grid {
            sum += row.iter().map(|x| *x as u64).sum::<u64>();
            if sum == half {
                return true;
            } else if sum > half {
                break;
            }
        }

        sum = 0;
        for c in 0..grid[0].len() {
            sum += &grid.iter().map(|row| row[c] as u64).sum::<u64>();
            if sum == half {
                return true;
            } else if sum > half {
                break;
            }
        }

        false
    }
}

fn main() {
    assert!(Solution::can_partition_grid(vec![vec![1, 4], vec![2, 3]]));
    assert!(!Solution::can_partition_grid(vec![vec![1, 3], vec![2, 4]]));
}
