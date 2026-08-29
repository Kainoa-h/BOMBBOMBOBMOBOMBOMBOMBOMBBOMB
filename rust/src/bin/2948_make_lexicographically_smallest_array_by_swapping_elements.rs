impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut indexes = (0..nums.len()).collect::<Vec<usize>>();
        indexes.sort_unstable_by_key(|&x| nums[x]);
        let mut result = vec![0; nums.len()];
        for val_ordered_idx in indexes.chunk_by(|&a, &b| nums[b] - nums[a] <= limit) {
            let mut index_ordered_idx = val_ordered_idx.to_vec();
            index_ordered_idx.sort_unstable();
            for (&src, idx) in val_ordered_idx.iter().zip(index_ordered_idx) {
                result[idx] = nums[src];
            }
        }
        result
    }
}

struct Solution {}
