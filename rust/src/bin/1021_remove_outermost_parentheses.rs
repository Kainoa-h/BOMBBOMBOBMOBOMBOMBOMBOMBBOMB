impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut parity = 0;
        s.chars()
            .filter(|&c| {
                let mut is_inside = parity > 0;
                parity += if c == '(' { 1 } else { -1 };
                if parity == 0 {
                    is_inside = false;
                }
                is_inside
            })
            .collect()
    }
}

struct Solution {}
