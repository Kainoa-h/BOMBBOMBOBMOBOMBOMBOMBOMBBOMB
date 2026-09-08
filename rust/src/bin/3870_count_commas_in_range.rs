impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        (n as u32).saturating_sub(999) as i32
    }
}

struct Solution {}
