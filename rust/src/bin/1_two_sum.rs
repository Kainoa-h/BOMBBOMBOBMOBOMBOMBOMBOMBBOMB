use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        match nums.into_iter().enumerate().try_fold(
            HashMap::<i32, usize>::with_capacity(nums.len()),
            |mut map, (idx, x)| {
                let diff = target - x;
                if let Some(&other_idx) = map.get(&diff) {
                    std::ops::ControlFlow::Break(vec![idx as i32, other_idx as i32])
                } else {
                    map.insert(x, idx);
                    std::ops::ControlFlow::Continue(map)
                }
            },
        ) {
            std::ops::ControlFlow::Break(x) => x,
            std::ops::ControlFlow::Continue(_) => vec![],
        }
    }
}
struct Solution {}
