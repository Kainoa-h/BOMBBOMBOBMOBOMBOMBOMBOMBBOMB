use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut available_letters: HashMap<u8, u32> = HashMap::new();
        for c in magazine.as_bytes() {
            *available_letters.entry(*c).or_default() += 1;
        }
        for c in ransom_note.as_bytes() {
            let Some(count) = available_letters.get_mut(&c) else {
                return false;
            };
            if *count == 0 {
                return false;
            }
            *count -= 1;
        }
        true
    }
}

fn main() {
    assert!(!Solution::can_construct("a".to_owned(), "b".to_owned()));
    assert!(!Solution::can_construct("aa".to_owned(), "ab".to_owned()));
    assert!(Solution::can_construct("aa".to_owned(), "aab".to_owned()));
}
