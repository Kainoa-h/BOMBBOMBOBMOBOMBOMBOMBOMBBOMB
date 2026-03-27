struct Solution {}
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        for row_idx in 0..board.len() {
            for col_idx in 0..board[0].len() {
                let neighbours = Solution::count_surroundings(board, row_idx, col_idx);
                if neighbours == 3 || (board[row_idx][col_idx] == 1 && neighbours == 2) {
                    board[row_idx][col_idx] |= 2;
                }
            }
        }
        for row in board.iter_mut() {
            for cell in row.iter_mut() {
                *cell >>= 1;
            }
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
                        .unwrap_or(0)
                        & 1;
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
