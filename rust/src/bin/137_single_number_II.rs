impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut bit_count = [0; 32];
        for n in nums {
            for (idx, count) in bit_count.iter_mut().enumerate() {
                if (n & (1 << idx)).count_ones() == 1 {
                    *count += 1;
                }
            }
        }

        let mut number = 0;
        for count in bit_count.into_iter().rev() {
            number <<= 1;
            if count % 3 != 0 {
                number |= 1;
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
