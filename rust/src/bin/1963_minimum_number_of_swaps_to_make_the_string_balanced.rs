impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let (_, errors) = s.chars().fold((0, 0), |(open, errors), c| match c {
            '(' => (open + 1, errors),
            _ if open > 0 => (open - 1, errors),
            _ => (open, errors + 1),
        });

        (errors + 1) / 2
    }
}

struct Solution {}
