impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 2 {
            return i32::from(nums[0] > nums[1]);
        }
        let acs = nums
            .chunk_by(|&a, &b| a + 1 == b)
            .map(|c| c.len())
            .collect::<Vec<_>>();
        let desc = nums
            .chunk_by(|&a, &b| a - 1 == b)
            .map(|c| c.len())
            .collect::<Vec<_>>();
        if acs.len() == 1 {
            return 0;
        }
        if desc.len() == 1 {
            return 1;
        }
        if acs.len() == 2 {
            let zero_pos = acs[0];
            return usize::min(zero_pos, n - zero_pos + 2) as i32;
        }
        if desc.len() == 2 {
            let last_pos = desc[0];
            return usize::min(last_pos + 1, n - last_pos + 1) as i32;
        }

        -1
    }
}

struct Solution {}
