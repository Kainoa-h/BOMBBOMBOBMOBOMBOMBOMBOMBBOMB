impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 {
            let mut bucket = [0; 51];
            for n in nums {
                bucket[n as usize] += 1;
            }
            bucket
                .iter()
                .enumerate()
                .filter(|&(_, &c)| c == 1)
                .map(|(n, _)| n as i32)
                .max()
                .unwrap_or(-1)
        } else if k == nums.len() as i32 {
            *nums.iter().max().unwrap_or(&-1)
        } else {
            let first = nums[0];
            let last = *nums.last().unwrap();
            if first == last {
                -1
            } else {
                let mut is_first = true;
                let mut is_last = true;

                for &n in &nums[1..nums.len() - 1] {
                    if n == first {
                        is_first = false;
                    } else if n == last {
                        is_last = false;
                    }
                }

                let first = if is_first { first } else { -1 };
                let last = if is_last { last } else { -1 };
                first.max(last)
            }
        }
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        -1,
        Solution::largest_integer(vec![4, 4, 2, 2, 2, 0, 5, 3, 4, 4], 3)
    );
}
