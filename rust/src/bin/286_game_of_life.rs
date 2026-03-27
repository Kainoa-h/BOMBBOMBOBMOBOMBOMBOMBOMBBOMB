struct Solution {}
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let mut dead_list: Vec<(usize, usize)> = Vec::new();
        let mut spawn_list: Vec<(usize, usize)> = Vec::new();

        for (row_idx, row) in board.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                let neighbours = Solution::count_surroundings(board, row_idx, col_idx);
                if cell == 1 {
                    if !(2..=3).contains(&neighbours) {
                        dead_list.push((row_idx, col_idx));
                    }
                } else if neighbours == 3 {
                    spawn_list.push((row_idx, col_idx));
                }
            }
        }
        for (row_idx, col_idx) in dead_list {
            board[row_idx][col_idx] = 0;
        }
        for (row_idx, col_idx) in spawn_list {
            board[row_idx][col_idx] = 1;
        }
    }

    fn count_surroundings(board: &[Vec<i32>], row_idx: usize, col_idx: usize) -> i32 {
        let mut sum = 0;
        let row_idx = row_idx as isize;
        let col_idx = col_idx as isize;

        for row_offset in -1..=1 {
            for col_offset in -1..=1 {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }

                let ri = row_idx + row_offset;
                let ci = col_idx + col_offset;

                if ri >= 0 && ci >= 0 {
                    sum += board
                        .get(ri as usize)
                        .and_then(|r| r.get(ci as usize))
                        .copied()
                        .unwrap_or(0);
                }
            }
        }
        sum
    }
}

fn what(ans: Vec<Vec<i32>>, mut board: Vec<Vec<i32>>) {
    println!("before: {:?}\n", board);
    Solution::game_of_life(&mut board);
    println!("after: {:?}\n", board);
    assert_eq!(ans, board);
}

fn main() {
    what(
        vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]],
        vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]],
    );
}
