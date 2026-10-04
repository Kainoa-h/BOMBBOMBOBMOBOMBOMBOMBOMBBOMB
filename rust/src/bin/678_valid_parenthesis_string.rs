impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut min_open = 0_u32;
        let mut max_open = 0_i32;
        for c in s.chars() {
            match c {
                '(' => {
                    max_open += 1;
                    min_open += 1;
                }
                ')' => {
                    min_open = min_open.saturating_sub(1);
                    max_open -= 1;
                    if max_open < 0 {
                        return false;
                    }
                },
                _ => {
                    min_open = min_open.saturating_sub(1);
                    max_open += 1;
                }
            }
        }
        min_open == 0
    }
}

struct Solution {}
