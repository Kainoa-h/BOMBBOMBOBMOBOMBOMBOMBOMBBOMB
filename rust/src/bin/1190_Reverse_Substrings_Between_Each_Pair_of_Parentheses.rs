impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = Vec::with_capacity(s.len() / 2);
        let mut result = Vec::with_capacity(s.len());

        for b in s.bytes() {
            match b {
                b'(' => stack.push(result.len()),
                b')' => result[stack.pop().unwrap()..].reverse(),
                _ => result.push(b),
            }
        }

        unsafe { String::from_utf8_unchecked(result) }
    }
}

struct Solution {}
