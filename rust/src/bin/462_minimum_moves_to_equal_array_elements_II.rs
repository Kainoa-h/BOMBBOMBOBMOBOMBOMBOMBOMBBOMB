impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        let m = nums.len() / 2;
        let (_, &mut mid, _) = nums.select_nth_unstable(m);
        nums.iter().map(|x| (x - mid).abs()).sum()
    }
}

struct Solution {}
