use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            if !Solution::row_map(&board, i) {
                return false;
            }
            if !Solution::col_map(&board, i) {
                return false;
            }
        }
        let dex = [0_usize, 3_usize, 6_usize];
        let row_iter = dex.iter();
        for r_id in row_iter {
            let col_iter = dex.iter();
            for c_id in col_iter {
                if !Solution::box_map(&board, *r_id, *c_id) {
                    return false;
                }
            }
        }
        true
    }

    fn box_map(board: &[Vec<char>], row_idx: usize, col_idx: usize) -> bool {
        let mut set: HashSet<char> = HashSet::new();
        for row_off in 0..3 {
            for coll_off in 0..3 {
                let c = board[row_idx + row_off][col_idx + coll_off];
                if c != '.' && !set.insert(c) {
                    return false;
                }
            }
        }
        true
    }

    fn col_map(board: &[Vec<char>], row_idx: usize) -> bool {
        let mut set: HashSet<char> = HashSet::new();
        for i in 0..9 {
            let c = board[i][row_idx];
            if c != '.' && !set.insert(c) {
                return false;
            }
        }
        true
    }

    fn row_map(board: &[Vec<char>], col_idx: usize) -> bool {
        let mut set: HashSet<char> = HashSet::new();
        let row: &Vec<char> = &board[col_idx];
        for c in row {
            if *c != '.' && !set.insert(*c) {
                return false;
            }
        }
        true
    }
}

fn what(board: Vec<Vec<char>>) {
    if Solution::is_valid_sudoku(board) {
        print!("nice");
    } else {
        print!("nay")
    }
    print!("\n---\n")
}

fn main() {
    what(vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ]);
    what(vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ]);
}
