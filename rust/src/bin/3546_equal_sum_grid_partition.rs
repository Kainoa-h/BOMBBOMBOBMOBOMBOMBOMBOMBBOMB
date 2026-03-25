struct Solution {}

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let rows = grid.len();
        println!("grid {:?}", grid);
        if rows > 1 {
            let flattened_rows: Vec<u64> = grid
                .iter()
                .map(|row| row.iter().map(|&x| x as u64).sum())
                .collect();
            let prefix_list = flattened_rows.iter().scan(0, |acc, x| {
                *acc += *x;
                Some(*acc)
            });
            let mut postfix_list: Vec<u64> = flattened_rows
                .iter()
                .rev()
                .scan(0_u64, |acc, &x| {
                    *acc = (*acc).checked_add(x).expect("overflow!");
                    Some(*acc)
                })
                .collect();
            postfix_list.pop();
            postfix_list.reverse();
            postfix_list.push(0);
            let postfix_list = postfix_list;

            for (a, &b) in prefix_list.zip(postfix_list.iter()) {
                if a == b {
                    return true;
                }
            }
        }

        let cols = grid[0].len();
        if cols > 1 {
            let flattned_cols: Vec<u64> = (0..cols)
                .map(|col_idx| grid.iter().map(|row| row[col_idx] as u64).sum())
                .collect();
            let prefix_list = flattned_cols.iter().scan(0_u64, |acc, &x| {
                *acc = (*acc).checked_add(x).expect("overflow!");
                Some(*acc)
            });
            let mut postfix_list: Vec<u64> = flattned_cols
                .iter()
                .rev()
                .scan(0_u64, |acc, x| {
                    *acc += *x;
                    Some(*acc)
                })
                .collect();
            postfix_list.pop();
            postfix_list.reverse();
            postfix_list.push(0);
            let postfix_list = postfix_list;

            for (a, &b) in prefix_list.zip(postfix_list.iter()) {
                if a == b {
                    return true;
                }
            }
        }

        false
    }
}

fn main() {
    // assert!(Solution::can_partition_grid(vec![vec![1, 4], vec![2, 3]]));
    assert!(!Solution::can_partition_grid(vec![vec![1, 3], vec![2, 4]]));
}
