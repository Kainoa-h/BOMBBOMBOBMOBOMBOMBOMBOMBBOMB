impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        if sum > 9 * num {
            return String::new();
        }

        let mut result = vec![b'0'; num as usize];
        let nines = (sum / 9) as usize;
        let remain = (sum % 9) as u8;
        result[0..nines].fill(b'9');
        if remain > 0 {
            result[nines] = b'0' + remain;
        }

        unsafe { String::from_utf8_unchecked(result) }
    }
}

struct Solution {}
