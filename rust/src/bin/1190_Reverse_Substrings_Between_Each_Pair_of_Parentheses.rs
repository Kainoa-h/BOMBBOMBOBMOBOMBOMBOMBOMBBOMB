impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut pairs = vec![0; s.len()];
        {
            let mut stack = Vec::with_capacity(s.len()/2);
            for (i, b) in s.bytes().enumerate() {
                match b {
                    b'(' => stack.push(i),
                    b')' => {
                        let open_idx = stack.pop().unwrap();
                        pairs[i] = open_idx;
                        pairs[open_idx] = i;
                    },
                    _ => {}
                }
            }
        }
        
        let mut result = Vec::with_capacity(s.len());
        let mut idx = 0;
        let mut direction = 1_isize;
        let bytes = s.as_bytes();
        while idx < bytes.len() {
            match bytes[idx] {
                b'(' | b')' => {
                    idx = pairs[idx];
                    direction = -direction;
                },
                b => result.push(b)
                
            }
            idx = idx.saturating_add_signed(direction);
        }
        
        unsafe { String::from_utf8_unchecked(result) }
    }
}

struct Solution {}
