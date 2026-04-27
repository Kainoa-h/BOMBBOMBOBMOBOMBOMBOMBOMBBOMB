use core::panic;
use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn has_valid_path(mut grid: Vec<Vec<i32>>) -> bool {
        let mut queue: VecDeque<(usize, usize, (isize, isize))> = VecDeque::new();

        const UP: (isize, isize) = (-1, 0);
        const RIGHT: (isize, isize) = (0, 1);
        const DOWN: (isize, isize) = (1, 0);
        const LEFT: (isize, isize) = (0, -1);

        let get_valid_dirs = |x| match x {
            1 => [LEFT, RIGHT],
            2 => [UP, DOWN],
            3 => [LEFT, DOWN],
            4 => [RIGHT, DOWN],
            5 => [UP, LEFT],
            6 => [UP, RIGHT],
            _ => panic!(),
        };

        match grid[0][0] {
            1 | 6 => queue.push_back((0, 0, RIGHT)),
            2 | 3 => queue.push_back((0, 0, DOWN)),
            4 => queue.extend([(0, 0, RIGHT), (0, 0, DOWN)]),
            _ => queue.push_back((0, 0, UP)),
        };

        while let Some((r, c, (dr, dc))) = queue.pop_front() {
            grid[r][c] = -1;
            if r == grid.len() - 1 && c == grid[0].len() - 1 {
                return true;
            }
            let Some((nr, nc)) = r.checked_add_signed(dr).zip(c.checked_add_signed(dc)) else {
                continue;
            };
            let Some(&next_node) = grid.get(nr).and_then(|row| row.get(nc)) else {
                continue;
            };
            if next_node == -1 {
                continue;
            }
            let receiving_dir = (-dr, -dc);
            let mut connected = false;
            let mut next_dir = (0, 0);
            for n_dir in get_valid_dirs(next_node) {
                if n_dir == receiving_dir {
                    connected = true;
                } else {
                    next_dir = n_dir;
                }
            }
            if connected {
                queue.push_back((nr, nc, next_dir));
            }
        }
        false
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
