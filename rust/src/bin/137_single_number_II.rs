impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ones = 0;
        let mut twos = 0;
        for n in nums {
            ones = (ones ^ n) & !twos;
            twos = (twos ^ n) & !ones;
        }
        ones
    }
}

struct Solution {}

fn main() {
    assert_eq!(99, Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]));
    assert_eq!(-4, Solution::single_number(vec![1, 1, 1, -2, -2, -2, -4]));
}
