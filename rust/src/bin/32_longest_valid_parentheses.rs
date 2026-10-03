impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = Vec::with_capacity(s.len());
        stack.push(-1);
        let mut longest = 0;
        for (idx, &byte) in s.as_bytes().iter().enumerate() {
            let idx = idx as isize;
            if byte == b'(' {
                stack.push(idx);
                continue;
            }
            stack.pop();
            match stack.last() {
                Some(&last)=> longest = longest.max(idx - last),
                None => stack.push(idx)
            }
        }
        longest as i32
    }
}

struct Solution {}
