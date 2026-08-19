use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        const SEATS_MASK_1: u16 = 0b01111000000;
        const SEATS_MASK_2: u16 = 0b00011110000;
        const SEATS_MASK_3: u16 = 0b00000111100;
        let mut row_mask_map = HashMap::<i32, u16>::new();
        for v in reserved_seats {
            let (row, col) = (v[0], v[1]);
            if (2..=9).contains(&col) {
                *row_mask_map.entry(row).or_insert(0) |= 1 << col;
            }
        }

        let sum = (n - row_mask_map.len() as i32) * 2;
        sum + row_mask_map
            .values()
            .filter(|&&row| {
                (row & SEATS_MASK_3) == 0 || (row & SEATS_MASK_2) == 0 || (row & SEATS_MASK_1) == 0
            })
            .count() as i32
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        4,
        Solution::max_number_of_families(
            3,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 8],
                vec![2, 6],
                vec![3, 1],
                vec![3, 10]
            ]
        )
    );
}
