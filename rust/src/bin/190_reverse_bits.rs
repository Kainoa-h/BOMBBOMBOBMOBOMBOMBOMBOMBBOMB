impl Solution {
    pub fn reverse_bits(mut n: i32) -> i32 {
        let mut result = 0;
        for _ in 0..32 {
            result = (result << 1) | (n & 1);
            n >>= 1;
        }
        result
    }
}

struct Solution {}
