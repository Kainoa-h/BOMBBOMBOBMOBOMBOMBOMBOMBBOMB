impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut x = 0;
        let mut y = 0;
        for (idx, n) in nums.into_iter().enumerate() {
            if idx % 2 == 0 {
                x += n;
            } else {
                y += n;
            }
        }
        x.max(y)
    }
}

struct Solution {}
