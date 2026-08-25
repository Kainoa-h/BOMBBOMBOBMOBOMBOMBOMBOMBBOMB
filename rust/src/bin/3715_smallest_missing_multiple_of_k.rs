impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut seen = [false;101];
        nums.into_iter().for_each(|x| seen[x as usize] = true);
        let mut x = k;
        while x < seen.len() && seen[x] {
            x += k;
        }
        x as i32
    }
}

struct Solution {}

fn main() {
     Solution::missing_multiple(vec![8,2,3,4,6], 2);
}
