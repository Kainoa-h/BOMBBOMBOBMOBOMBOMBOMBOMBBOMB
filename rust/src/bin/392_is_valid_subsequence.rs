struct Solution {}
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() > t.len() {
            return false;
        }

        let mut idx = 0_usize;
        for &c in t.as_bytes() {
            if idx == s.len() {
                break;
            }

            if c == s.as_bytes()[idx] {
                idx += 1;
            }
        }

        idx == s.len()
    }
}

fn main() {
    assert!(Solution::is_subsequence(
        "abc".to_owned(),
        "ahbgdc".to_owned()
    ));
    assert!(!Solution::is_subsequence(
        "axc".to_owned(),
        "ahbgdc".to_owned()
    ));
}
