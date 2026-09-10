impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut res = vec![nums[0]];
        for n in nums.into_iter().skip(1) {
            if res[res.len()-1] < n {
                res.push(n);
            } else {
                let idx = res.partition_point(|&x| x < n);
                res[idx] = n;
            }

        }
        res.len() as i32
    }
}

struct Solution {}
