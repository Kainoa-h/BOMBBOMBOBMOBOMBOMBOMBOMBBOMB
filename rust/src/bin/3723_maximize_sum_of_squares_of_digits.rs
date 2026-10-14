impl Solution {
    pub fn max_sum_of_squares(mut num: i32, mut sum: i32) -> String {
        if sum > 9 * num {
            return String::new();
        }

        let mut result = String::with_capacity(num as usize);
        let nines = sum / 9;
        let remain = sum % 9;
        (0..nines).for_each(|_| result.push('9'));
        if remain > 0 {
            result.push((b'0' + remain as u8) as char);
        }
        (result.len()..num as usize).for_each(|_| result.push('0'));

        result
    }
}

struct Solution {}
