struct Solution {}
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }

        n ^ ((u32::MAX) >> n.leading_zeros()) as i32
    }
}

fn main() {
    assert_eq!(1, Solution::bitwise_complement(0));
    assert_eq!(1, Solution::bitwise_complement(2));
    assert_eq!(0, Solution::bitwise_complement(7));
}
