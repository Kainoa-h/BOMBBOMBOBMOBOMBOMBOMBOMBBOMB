impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut v1 = Vec::with_capacity(nums.len());
        let mut v2 = Vec::new();

        v1.push(nums[0]);
        v2.push(nums[1]);

        for r in nums.into_iter().skip(2) {
            if v1.last().unwrap() > v2.last().unwrap() {
                v1.push(r)
            } else {
                v2.push(r)
            }
        }

        v1.append(&mut v2);
        v1
    }
}

struct Solution {}
