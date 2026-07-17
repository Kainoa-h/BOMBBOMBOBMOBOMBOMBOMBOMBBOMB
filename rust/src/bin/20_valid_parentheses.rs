impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for &c in s.as_bytes() {
            match c {
                b'[' => stack.push(b']'),
                b'{' => stack.push(b'}'),
                b'(' => stack.push(b')'),
                _ => {
                    if Some(c) != stack.pop() {
                        return false;
                    }
                }
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
