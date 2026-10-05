impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = Vec::with_capacity(s.len()/2);
        let mut bytes = s.into_bytes();
        for i in 0..bytes.len() {
            let b = bytes[i];
            match b {
                b'(' => stack.push(i),
                b')' => {
                    let start = stack.pop().unwrap() + 1;
                    bytes[start..i].reverse();
                },
                _ => {}
            }
        }
        String::from_utf8(
            bytes.into_iter().filter(|b| (b'a'..=b'z').contains(b)).collect()
        ).unwrap()
    }
}

struct Solution {}
