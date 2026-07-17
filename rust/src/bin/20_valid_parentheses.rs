impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for &c in s.as_bytes() {
            if c == b'[' || c == b'{' || c == b'(' {
                stack.push(c);
                continue;
            }
            let top = stack.pop();
            if top.is_none() || c != match top.unwrap() {
                    b'[' => b']',
                    b'{' => b'}',
                    b'(' => b')',
                    _ => b' ',
                }
            {
                return false;
            }
        }

        stack.is_empty()
    }
}

struct Solution {}

fn main() {
    assert!(!Solution::is_valid("(}".to_string()));
    assert!(Solution::is_valid("()[{()}]{}".to_string()));
    assert!(!Solution::is_valid("(".to_string()));
    assert!(!Solution::is_valid("}".to_string()));
}
