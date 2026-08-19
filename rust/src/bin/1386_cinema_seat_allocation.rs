impl Solution {
    pub fn max_number_of_families(mut n: i32, mut reserved_seats: Vec<Vec<i32>>) -> i32 {
        reserved_seats.sort_unstable_by_key(|v| (v[0], v[1]));

        let mut sum = 0;
        for chunk in reserved_seats.chunk_by(|a, b| a[0] == b[0]) {
            let mut row = [true; 10];
            for c in chunk {
                row[c[1] as usize - 1] = false;
            }
            let avail = row[1] && row[2] && row[3] && row[4];
            sum += avail as usize;
            row[4] &= !avail;
            let avail = row[3] && row[4] && row[5] && row[6];
            sum += avail as usize;
            row[6] &= !avail;
            sum += (row[5] && row[6] && row[7] && row[8]) as usize;

            n -= 1;
        }
        sum as i32 + n * 2
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
