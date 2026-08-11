impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let (first, rest) = nums.split_first().unwrap();
        let mut prev = *first;
        let mut sum = *first;
        let mut left = 0_usize;
        let mut valid_nums = [true; 51];
        valid_nums[sum as usize] = false;
        for (idx,&n) in rest.iter().enumerate() {
            if n != prev + 1 {
                left = idx + 1;
                break;
            }
            prev = n;
            sum += n;
            valid_nums[n as usize] = false;
        }

        if sum > 50 {
            return sum;
        }

        for &n in rest.iter().skip(left.saturating_sub(1)) {
            valid_nums[n as usize] = false;
        }

        for (idx, &b) in valid_nums.iter().enumerate().skip(sum as usize) {
            if b {
                return idx as i32;
            }
        }

        51
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::missing_integer(vec![49,27,5,45,50,6,45,28,6,11,8,7,35,20]),
        51
    );
}
