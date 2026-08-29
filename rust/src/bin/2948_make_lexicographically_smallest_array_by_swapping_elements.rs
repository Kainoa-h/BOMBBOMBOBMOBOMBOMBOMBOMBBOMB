impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut indexes = (0..nums.len()).collect::<Vec<usize>>();
        indexes.sort_unstable_by_key(|&x| nums[x]);
        let mut result = vec![0; nums.len()];
        for group in indexes.chunk_by(|&a, &b| nums[b] - nums[a] <= limit) {
            let mut index_order_group = group.to_vec();
            index_order_group.sort_unstable_by_key(|&x| x);
            for i in 0..group.len() {
                let val = nums[group[i]];
                let idx = index_order_group[i];
                result[idx] = val;
            }
        }
        result
    }
}

struct Solution {}
