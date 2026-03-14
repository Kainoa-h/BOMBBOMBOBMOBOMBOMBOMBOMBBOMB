use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        let mut start_idx = 0;
        let mut char_set: HashMap<char, usize> = HashMap::new();
        for (end_index, char) in s.char_indices() {
            if let Some(old_dup_idx) = char_set.insert(char, end_index) {
                longest = longest.max(char_set.len());
                for idx in start_idx..old_dup_idx {
                    let c = s.as_bytes()[idx] as char;
                    char_set.remove(&c);
                }

                start_idx = old_dup_idx + 1;
                continue;
            }
        }
        longest = longest.max(char_set.len());
        longest as i32
    }
}

fn main() {
    assert_eq!(
        3,
        Solution::length_of_longest_substring("abcabcbb".to_string())
    );
    assert_eq!(
        8,
        Solution::length_of_longest_substring("abcadefgh".to_string())
    );
    assert_eq!(
        4,
        Solution::length_of_longest_substring("abccbad".to_string())
    );
}
