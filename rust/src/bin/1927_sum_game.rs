impl Solution {
    pub fn sum_game(num: String) -> bool {
        let mid = num.len() / 2;
        let digits = num.as_bytes();

        let foo = |span: &[u8]| -> (i32,i32) {
            span.iter().fold((0,0), |acc, &x| {
                if x == b'?' {
                    (acc.0 + 1, acc.1)
                } else {
                    (acc.0, acc.1 + (x - b'0') as i32)
                }
            })
        };

        let (left_q, left_sum) = foo(&digits[..mid]);
        let (right_q, right_sum) = foo(&digits[mid..]);

        (left_q + right_q) % 2 == 1 || left_sum - right_sum != ((right_q - left_q) * 9) / 2
    }
}

struct Solution {}
