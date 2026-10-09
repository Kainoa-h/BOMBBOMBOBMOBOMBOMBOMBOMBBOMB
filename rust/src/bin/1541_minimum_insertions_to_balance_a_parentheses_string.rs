impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut insertions = 0;
        let mut needed_right = 0;

        for &b in s.as_bytes() {
            if b == b'(' {
                if needed_right % 2 != 0 {
                    insertions += 1;
                    needed_right -= 1;
                }
                needed_right += 2;
            } else {
                needed_right -= 1;
                if needed_right < 0 {
                    insertions += 1;
                    needed_right += 2;
                }
            }
        }

        insertions + needed_right
    }
}

struct Solution {}
