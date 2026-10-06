impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let (open, needed) = s.chars().fold((0, 0), |(open, needed), c| match c {
            '(' => (open + 1, needed),
            _ if open > 0 => (open - 1, needed),
            _ => (open, needed + 1),
        });
        open + needed
    }
}

struct Solution {}
