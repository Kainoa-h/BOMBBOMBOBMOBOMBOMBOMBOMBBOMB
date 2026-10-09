impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut insertions = 0; 
        let mut depth = 0;
        for c in s.as_bytes().chunk_by(|a, b| a == b) {
            let mut num = c.len();
            depth += if c[0] == b'(' {
                num as i32
            } else {
                insertions += i32::from(num % 2 != 0);
                -(num.div_ceil(2) as i32)
            };
            if depth < 0 {
                insertions += -depth;
                depth = 0;
            }
        }

        insertions + depth * 2
    }
}

struct Solution { }
