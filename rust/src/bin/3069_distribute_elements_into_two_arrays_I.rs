impl Solution {
    pub fn result_array(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut v2 = Vec::with_capacity(n);

        let mut l1 = nums[0];
        let mut l2 = nums[1];

        v2.push(l2);
        let mut v1_len = 1;

        for i in 2..n {
            let x = nums[i];
            if l1 > l2 {
                nums[v1_len] = x;
                v1_len += 1;
                l1 = x;
            }
            else {
                v2.push(x);
                l2 = x;
            }
        }

        nums.truncate(v1_len);
        nums.append(&mut v2);
        nums
    }
}

struct Solution {}
