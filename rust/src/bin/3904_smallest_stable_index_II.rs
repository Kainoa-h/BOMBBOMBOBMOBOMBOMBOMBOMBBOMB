impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let Some(mut largest) = nums.first().copied() else {
            return -1;
        };

        let mut postfix = nums.clone();
        for i in (1..postfix.len()).rev() {
            postfix[i - 1] = postfix[i].min(postfix[i - 1]);
        }

        for (idx, (n, smallest)) in nums.into_iter().zip(postfix).enumerate() {
            largest = largest.max(n);
            if largest - smallest <= k {
                return idx as i32;
            }
        }
        -1
    }
}
struct Solution {}

fn main() {
    Solution::first_stable_index(vec![], 0);
}
