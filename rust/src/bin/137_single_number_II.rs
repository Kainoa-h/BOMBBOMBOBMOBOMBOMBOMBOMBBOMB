impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut bit_count = [0; 32];
        for n in nums {
            for (idx, count) in bit_count.iter_mut().enumerate() {
                *count += (n >> idx) & 1;
            }
        }

        let mut number = 0;
        for (idx,count) in bit_count.into_iter().enumerate() {
            if count % 3 != 0 {
                number |= 1 << idx;
            }
        }
        number
    }
}

struct Solution {}

fn main() {
    assert_eq!(99, Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]));
    assert_eq!(-4, Solution::single_number(vec![1, 1, 1, -2, -2, -2, -4]));
}
