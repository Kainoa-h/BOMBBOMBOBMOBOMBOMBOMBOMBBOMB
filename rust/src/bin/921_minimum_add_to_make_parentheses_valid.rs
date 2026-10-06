impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let (mut open, mut close) = (0, 0);
        let mut result = 0;
        for ch in s.chars() {
            match ch {
                '(' => open += 1,
                _ => close += 1
            }
            if close > open {
                result += 1;
                open = 0;
                close = 0;
            }
        }
        result + open - close
    }
}

struct Solution {}
