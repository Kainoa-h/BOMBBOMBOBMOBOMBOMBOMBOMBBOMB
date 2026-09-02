impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let rows = board.len();
        let cols = board[0].len();

        fn mark_safe(board: &mut [Vec<char>], r: usize, c: usize) {
            board[r][c] = 'S';
            for (mr, mc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (nr, nc) = (r as isize + mr, c as isize + mc);
                if (0..board.len() as isize).contains(&nr)
                    && (0..board[0].len() as isize).contains(&nc)
                    && board[nr as usize][nc as usize] == 'O'
                {
                    mark_safe(board, nr as usize, nc as usize);
                }
            }
        }

        for i in 0..rows {
            if board[i][0] == 'O' {
                mark_safe(board, i, 0);
            }
            if board[i][cols - 1] == 'O' {
                mark_safe(board, i, cols - 1);
            }
        }

        for i in 1..cols - 1 {
            if board[0][i] == 'O' {
                mark_safe(board, 0, i);
            }
            if board[rows - 1][i] == 'O' {
                mark_safe(board, rows - 1, i);
            }
        }

        for row in board {
            for cell in row {
                match *cell {
                    'S' => *cell = 'O',
                    'O' => *cell = 'X',
                    _ => {}
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
