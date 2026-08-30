impl Solution {
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut hamming_weight = 0;
        for _ in 0..32 {
            hamming_weight += n & 1;
            n >>= 1;
        }
        hamming_weight
    }
}

struct Solution{}
