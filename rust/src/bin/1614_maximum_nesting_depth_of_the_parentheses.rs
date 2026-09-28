impl Solution {
    pub fn max_depth(s: String) -> i32 {
        s.chars().scan(0, |acc, c| {
            *acc += i32::from(c == '(');
            *acc -= i32::from(c == ')');
            Some(*acc)
        }).max().unwrap_or(0)
    }
}

struct Solution {}
