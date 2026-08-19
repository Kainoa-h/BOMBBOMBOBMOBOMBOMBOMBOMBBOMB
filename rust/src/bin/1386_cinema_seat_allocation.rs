impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut seats = vec![vec![true; 10]; n as usize];
        for v in reserved_seats {
            let (row, col) = (v[0] as usize - 1, v[1] as usize - 1);
            seats[row][col] = false;
        }

        let mut sum = 0;
        for row in seats.iter_mut() {
            let avail = row[1] && row[2] && row[3] && row[4];
            sum += avail as usize;
            row[4] &= !avail;
            let avail = row[3] && row[4] && row[5] && row[6];
            sum += avail as usize;
            row[6] &= !avail;
            sum += (row[5] && row[6] && row[7] && row[8]) as usize;
            println!("row {:?}", row);
            println!("sum {}", sum);
        }


        sum as i32
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
