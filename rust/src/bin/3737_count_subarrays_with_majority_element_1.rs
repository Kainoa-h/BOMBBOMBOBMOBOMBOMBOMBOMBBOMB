impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let total_targets = nums.iter().filter(|&&x| x == target).count() as i32;
        if total_targets == 0 {
            return 0;
        }
        let prefix = nums
            .iter()
            .scan(0, |acc, &x| {
                let curr = *acc;
                *acc += (x == target) as i32;
                Some(curr)
            })
            .collect::<Vec<i32>>();
        let mut suffix = nums
            .iter()
            .rev()
            .scan(0, |acc, &x| {
                let curr = *acc;
                *acc += (x == target) as i32;
                Some(curr)
            })
            .collect::<Vec<i32>>();
        suffix.reverse();

        let mut count = 0;
        for start_inc in 0..nums.len() {
            for end_inc in start_inc..nums.len() {
                let target_count = total_targets - prefix[start_inc] - suffix[end_inc];
                let len = (end_inc - start_inc) as i32 + 1;
                count += (target_count * 2 > len) as i32;
            }
        }

        count
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::count_majority_subarrays(vec![1, 2, 2, 3], 2), 5);
}
