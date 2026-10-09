impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut insertions = 0; 
        let mut depth = 0;
        let mut chars_iter = s.bytes().peekable();
        while let Some(c) = chars_iter.next() {
            if c == b'(' {
                depth += 1;
            } else {
                if chars_iter.next_if_eq(&c).is_none() {
                    insertions += 1;
                }
                if depth > 0 {
                    depth -= 1;
                } else {
                    insertions += 1;
                }
            }
        }

        insertions + depth * 2
    }
}

struct Solution { }
