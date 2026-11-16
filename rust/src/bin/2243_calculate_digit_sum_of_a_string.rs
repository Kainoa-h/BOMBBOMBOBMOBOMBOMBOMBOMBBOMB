use std::fmt::Write as _;
impl Solution {
    pub fn digit_sum(mut s: String, k: i32) -> String {
        let k = k as usize;
        let mut sb = String::new();
        while s.len() > k {
            for chunk in s.as_bytes().chunks(k) {
                let sum = chunk.iter().map(|&x| (x - b'0') as u32).sum::<u32>();
                let _ = write!(sb, "{sum}");
            }
            std::mem::swap(&mut s, &mut sb);
            sb.clear();
        }
        s
    }
}

struct Solution {}
