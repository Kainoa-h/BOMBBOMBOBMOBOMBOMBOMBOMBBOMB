struct Solution {}
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut mask = n;
        mask |= mask >> 1;
        mask |= mask >> 2;
        mask |= mask >> 4;
        mask |= mask >> 8;
        mask |= mask >> 16;

        n ^ mask
    }
}

fn main() {
    assert_eq!(1, Solution::bitwise_complement(0));
    assert_eq!(1, Solution::bitwise_complement(2));
    assert_eq!(0, Solution::bitwise_complement(7));
}
