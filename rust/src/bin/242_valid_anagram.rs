struct Solution {}
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut counter = [0_u32; 256];
        for c in s.bytes() {
            counter[c as usize] += 1;
        }

        for c in t.bytes() {
            let i = c as usize;
            if counter[i] == 0 {
                return false;
            }
            counter[i] -= 1;
        }

        true
    }
}

fn main() {
    assert!(Solution::is_anagram(
        "anagram".to_owned(),
        "nagaram".to_owned()
    ));

    assert!(!Solution::is_anagram("rat".to_owned(), "car".to_owned()));

    assert!(!Solution::is_anagram("am".to_owned(), "amm".to_owned()));
}
