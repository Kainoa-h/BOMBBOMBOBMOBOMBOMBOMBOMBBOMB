impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let mut versioned_seen = [0_usize; 101];
        let mut result = 0;
        for i in 0..nums.len() {
            let mut distinct = 0_i32;
            for &x in nums.iter().skip(i) {
                if versioned_seen[x as usize] != i + 1 {
                    versioned_seen[x as usize] = i + 1;
                    distinct += 1;
                }
                result += distinct.pow(2);
            }
        }

        result
    }
}

struct Solution {}
