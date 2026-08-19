impl Solution {
    pub fn max_number_of_families(mut n: i32, mut reserved_seats: Vec<Vec<i32>>) -> i32 {
        const SEATS_MASK_1: u16 = 0b0111100000;
        const SEATS_MASK_2: u16 = 0b0001111000;
        const SEATS_MASK_3: u16 = 0b0000011110;

        reserved_seats.sort_unstable_by_key(|v| v[0]);

        let mut sum = 0;
        for chunk in reserved_seats.chunk_by(|a, b| a[0] == b[0]) {
            let mut row = 0_u16;
            for c in chunk {
                row |= 1 << (c[1] - 1);
            }
            let left = row & SEATS_MASK_1 == 0;
            let mid = row & SEATS_MASK_2 == 0;
            let right = row & SEATS_MASK_3 == 0;

            if left && right {
                sum += 2;
            }
            else if left || mid || right {
                sum += 1;
            }

            n -= 1;
        }
        sum + n * 2
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
