impl Solution {
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort_unstable();
        nums.iter()
            .enumerate()
            .filter_map(|(i, &x)| (x == target).then_some(i as i32))
            .collect::<Vec<i32>>()
    }
}

struct Solution {}
