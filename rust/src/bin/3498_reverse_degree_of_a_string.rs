impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        s.as_bytes().iter().enumerate().fold(0, |acc, (idx, &b)| {
            acc + ((idx as i32 + 1) * (b'z' - b + 1) as i32)
        })
    }
}

struct Solution {}
