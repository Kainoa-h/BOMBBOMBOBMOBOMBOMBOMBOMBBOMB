impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut depth = 0;
        let mut score = 0;
        for (idx, &b) in bytes.iter().enumerate() {
            match b {
                b'(' => depth += 1,
                _ => {
                    depth -= 1;
                    if idx > 0 && bytes[idx - 1] == b'(' {
                        score += 1 << depth;
                    }
                }
            }
        }
        score
    }
}

struct Solution {}
