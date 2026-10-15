impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..nums.len() {
            let mut set = [true; 101];
            let mut distinct = 0_i32;
            for &x in nums.iter().skip(i) {
                if set[x as usize] {
                    distinct += 1;
                }
                set[x as usize] = false;
                result += distinct.pow(2);
            }
        }

        result
    }
}

struct Solution {}
