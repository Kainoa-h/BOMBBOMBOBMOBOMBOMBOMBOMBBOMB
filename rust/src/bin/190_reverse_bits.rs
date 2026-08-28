impl Solution {
    pub fn reverse_bits(mut n: i32) -> i32 {
        let mut result = 0;
        for _ in 0..32 {
            result <<= 1;
            let x = n & 1;
            result |= x;
            n >>= 1;
        }
        result
    }
}

struct Solution {}
