impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let rows = board.len();
        let cols = board[0].len();
        let mut visited_board = vec![vec![false; cols]; rows];

        fn can_capture(
            board: &mut Vec<Vec<char>>,
            visited_board: &mut Vec<Vec<bool>>,
            r: usize,
            c: usize,
            rows: usize,
            cols: usize,
        ) -> bool {
            visited_board[r][c] = true;

            let mut can = true;
            for (rm, cm) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (nr, nc) = (r as isize + rm, c as isize + cm);
                if (0..rows as isize).contains(&nr) && (0..cols as isize).contains(&nc) {
                    if !visited_board[nr as usize][nc as usize]
                        && board[nr as usize][nc as usize] == 'O' {
                        can &= can_capture(board, visited_board, nr as usize, nc as usize, rows, cols)
                    }
                } else {
                    can = false;
                }
            }
            can
        }

        fn capture(board: &mut Vec<Vec<char>>, r: usize, c: usize, rows: usize, cols: usize) {
            board[r][c] = 'X';
            for (rm, cm) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (nr, nc) = (r as isize + rm, c as isize + cm);
                if (0..rows as isize).contains(&nr)
                    && (0..cols as isize).contains(&nc)
                    && board[nr as usize][nc as usize] == 'O'
                {
                    capture(board, nr as usize, nc as usize, rows, cols);
                }
            }
        }

        for r in 1..rows - 1 {
            for c in 1..cols - 1 {
                let letter = board[r][c];
                if letter == 'O'
                    && !visited_board[r][c]
                    && can_capture(board, &mut visited_board, r, c, rows, cols)
                {
                    capture(board, r, c, rows, cols);
                }
            }
        }
    }
}

struct Solution {}

fn main() {
    let mut v = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    Solution::solve(&mut v);
    println!("{:?}", v);
}
