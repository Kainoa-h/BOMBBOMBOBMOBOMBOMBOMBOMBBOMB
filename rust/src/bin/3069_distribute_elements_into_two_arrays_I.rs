impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut v1 = Vec::with_capacity(nums.len());
        let mut v2 = Vec::new();

        let mut l1 = nums[0];
        let mut l2 = nums[1];

        v1.push(l1);
        v2.push(l2);

        for r in nums.into_iter().skip(2) {
            if l1 > l2 {
                l1 = r;
                v1.push(r);
            } else {
                l2 = r;
                v2.push(r);
            }
        }

        v1.append(&mut v2);
        v1
    }
}

struct Solution {}
