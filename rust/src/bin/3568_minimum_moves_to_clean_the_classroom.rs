use std::collections::{HashMap, VecDeque};

struct State {
    row: usize,
    col: usize,
    steps: u32,
    energy: u16,
    mask: u16,
}

impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        let mut litter_map = HashMap::with_capacity(10);
        let mut litter_mask = 0_u16;
        let mut count = 0;
        let (mut start_row, mut start_col) = (0, 0);
        for (row_i, row) in classroom.iter().enumerate() {
            for (col_i, &byte) in row.as_bytes().iter().enumerate() {
                if byte == b'L' {
                    litter_map.insert((row_i, col_i), count);
                    litter_mask |= 1 << count;
                    count += 1;
                } else if byte == b'S' {
                    start_row = row_i;
                    start_col = col_i;
                }
            }
        }

        let rows = classroom.len();
        let cols = classroom[0].len();
        let mut best_mask = vec![vec![HashMap::<u16, u16>::new(); cols]; rows];
        let mut queue = VecDeque::<State>::new();
        queue.push_back(State {
            row: start_row,
            col: start_col,
            steps: 0,
            energy: energy as u16,
            mask: 0,
        });

        while let Some(mut state) = queue.pop_front() {
            match classroom[state.row].as_bytes()[state.col] {
                b'X' => continue,
                b'R' => state.energy = energy as u16,
                b'L' => {
                    state.mask |= 1 << *litter_map.get(&(state.row, state.col)).unwrap_or(&0) as u16
                }
                _ => {}
            }

            if state.mask == litter_mask {
                return state.steps as i32;
            }

            let entry = best_mask[state.row][state.col]
                .entry(state.mask)
                .or_insert(0);

            if *entry >= state.energy {
                continue;
            }
            *entry = state.energy;

            if state.energy == 0 {
                continue;
            }
            if state.row > 0 {
                queue.push_back(State {
                    row: state.row - 1,
                    col: state.col,
                    steps: state.steps + 1,
                    energy: state.energy - 1,
                    mask: state.mask,
                });
            }
            if state.row < rows - 1 {
                queue.push_back(State {
                    row: state.row + 1,
                    col: state.col,
                    steps: state.steps + 1,
                    energy: state.energy - 1,
                    mask: state.mask,
                });
            }
            if state.col > 0 {
                queue.push_back(State {
                    row: state.row,
                    col: state.col - 1,
                    steps: state.steps + 1,
                    energy: state.energy - 1,
                    mask: state.mask,
                });
            }
            if state.col < cols - 1 {
                queue.push_back(State {
                    row: state.row,
                    col: state.col + 1,
                    steps: state.steps + 1,
                    energy: state.energy - 1,
                    mask: state.mask,
                });
            }
        }
        -1
    }
}

struct Solution {}
