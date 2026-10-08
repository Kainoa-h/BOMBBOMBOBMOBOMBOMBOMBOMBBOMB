impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut depth = 0;
        s.chars()
            .filter(|&c| match c {
                '(' => {
                    depth += 1;
                    depth > 1
                }
                ')' => {
                    depth -= 1;
                    depth > 0
                }
                _ => unreachable!(),
            })
            .collect()
    }
}

struct Solution {}
