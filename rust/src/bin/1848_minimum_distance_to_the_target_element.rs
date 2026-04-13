struct Solution {}
impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let start = start as usize;
        let mut i = 0_usize;
        loop {
            let left = start.checked_sub(i).and_then(|idx| nums.get(idx)) == Some(&target);
            let right = nums.get(start + i) == Some(&target);
            if left || right {
                return i as i32;
            }
            i += 1;
        }
    }
}

fn main() {}
