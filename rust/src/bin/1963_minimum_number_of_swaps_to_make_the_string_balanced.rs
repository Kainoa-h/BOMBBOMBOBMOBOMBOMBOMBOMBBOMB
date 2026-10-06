impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut open = 0;
        let mut max_diff = 0;
        for &c in s.as_bytes() {
            match c {
                b'[' => open += 1,
                _ => open -= 1
            }
            max_diff = max_diff.min(open);
        }
        (-max_diff + 1) / 2
    }
}

struct Solution {}
