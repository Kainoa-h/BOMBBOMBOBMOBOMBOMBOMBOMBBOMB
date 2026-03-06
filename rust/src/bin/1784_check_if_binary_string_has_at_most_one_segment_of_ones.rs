struct Solution {}

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let mut in_sequence = true;
        for c in s.bytes() {
            if in_sequence {
                if c == b'0' {
                    in_sequence = false;
                }
            } else if c == b'1' {
                return false;
            }
        }
        true
    }
}

fn main() {}
