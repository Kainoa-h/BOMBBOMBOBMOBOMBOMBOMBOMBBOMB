impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = Vec::with_capacity(s.len());
        for ch in s.chars() {
            if ch == '(' {
                stack.push(0);
                continue;
            }
            let mut score = 0;
            while let Some(x) = stack.pop() && x != 0 {
                score += x;
            }
            score *= 2;
            stack.push(score.max(1));
        }
        stack.iter().sum()
    }
}

struct Solution {}
