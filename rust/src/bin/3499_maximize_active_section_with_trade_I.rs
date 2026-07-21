impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let mut total_ones = 0;
        let mut max_delta = 0;
        let mut prev_zeros = 0;
        for chunk in s.as_bytes().chunk_by(|a, b| a == b) {
            let len = chunk.len();
            if chunk[0] == b'1' {
                total_ones += len;
                continue;
            }
            if prev_zeros != 0 {
                max_delta = max_delta.max(len + prev_zeros);
            }
            prev_zeros = len;
        }
        (total_ones + max_delta) as i32
    }
}

struct Solution {}
