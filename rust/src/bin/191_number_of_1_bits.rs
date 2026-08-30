impl Solution {
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut hamming_weight = 0;
        while n > 0 {
            n &= n - 1;
            hamming_weight += 1;
        }
        hamming_weight
    }
}

struct Solution {}
