impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        s.chars()
            .fold(0_i32, |acc, c| acc + if c == '(' { 1 } else { -1 })
            .abs()
    }
}

struct Solution {}
