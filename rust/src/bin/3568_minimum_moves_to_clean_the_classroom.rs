use std::collections::{HashMap, VecDeque};

struct State {
    row: usize,
    col: usize,
    energy: u16,
    mask: u16,
}

impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        let mut litter_map = HashMap::with_capacity(10);
        let mut count = 0;
        let mut start = (0, 0);
        for (row_i, row) in classroom.iter().enumerate() {
            for (col_i, &byte) in row.as_bytes().iter().enumerate() {
                if byte == b'L' {
                    litter_map.insert((row_i, col_i), count);
                    count += 1;
                } else if byte == b'S' {
                    start = (row_i, col_i);
                }
            }
        }
        let litter_mask = (1 << count) - 1;

        let rows = classroom.len();
        let cols = classroom[0].len();
        let mut best_mask = vec![vec![HashMap::<u16, u16>::new(); cols]; rows];
        let mut queue = VecDeque::<State>::new();
        queue.push_back(State {
            row: start.0,
            col: start.1,
            energy: energy as u16,
            mask: 0,
        });

        let mut steps = 0;
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let mut state = queue.pop_front().unwrap();
                match classroom[state.row].as_bytes()[state.col] {
                    b'R' => state.energy = energy as u16,
                    b'L' => {
                        state.mask |=
                            1 << *litter_map.get(&(state.row, state.col)).unwrap_or(&0) as u16
                    }
                    _ => {}
                }

                if state.mask == litter_mask {
                    return steps;
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

                for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nr = state.row as isize + dr;
                    let nc = state.col as isize + dc;

                    if nr >= 0
                        && nr < rows as isize
                        && nc >= 0
                        && nc < cols as isize
                        && classroom[nr as usize].as_bytes()[nc as usize] != b'X'
                    {
                        queue.push_back(State {
                            row: nr as usize,
                            col: nc as usize,
                            energy: state.energy - 1,
                            mask: state.mask,
                        });
                    }
                }
            }
            steps += 1;
        }
        -1
    }
}

struct Solution {}
