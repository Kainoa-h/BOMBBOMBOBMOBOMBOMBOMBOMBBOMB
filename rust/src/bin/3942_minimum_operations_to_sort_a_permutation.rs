impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let Some(zero) = nums.iter().position(|&x| x == 0) else {
            return -1;
        };
        let is_cont = |s1: &[i32], s2: &[i32], d: i32| {
            s1.windows(2).all(|w| w[0] - w[1] == d) && s2.windows(2).all(|w| w[0] - w[1] == d)
        };
        let asc = is_cont(&nums[0..zero], &nums[zero..], -1);
        let desc = is_cont(&nums[0..=zero], &nums[zero + 1..], 1);

        if asc {
            return usize::min(zero, n - zero + 2) as i32;
        }

        if desc {
            return usize::min(zero + 2, n - zero) as i32;
        }

        -1
    }
}

struct Solution {}
