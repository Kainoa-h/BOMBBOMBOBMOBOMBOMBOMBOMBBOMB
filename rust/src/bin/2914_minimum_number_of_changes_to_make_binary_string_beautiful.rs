impl Solution {
    pub fn min_changes(s: String) -> i32 {
        s.as_bytes()
            .chunks_exact(2)
            .map(|c| i32::from(c[0] != c[1]))
            .sum()
    }
}

struct Solution {}
