use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut char_map = [None::<&str>; 128];
        let mut word_map: HashMap<&str, u8> = HashMap::new();
        let words = s.split_ascii_whitespace().collect::<Vec<&str>>();

        if words.len() != pattern.len() {
            return false;
        }

        for (char, word) in pattern.bytes().zip(words) {
            let ce = char_map.get_mut(char as usize).unwrap().get_or_insert(word);
            if *ce != word {
                return false;
            }

            let we = word_map.entry(word).or_insert(char);
            if *we != char {
                return false;
            }
        }

        true
    }
}

fn main() {
    assert!(Solution::word_pattern(
        "abba".to_owned(),
        "dog cat cat dog".to_owned()
    ));

    assert!(!Solution::word_pattern(
        "abba".to_owned(),
        "dog cat cat fish".to_owned()
    ));

    assert!(!Solution::word_pattern(
        "aaaa".to_owned(),
        "dog cat cat dog".to_owned()
    ));

    assert!(!Solution::word_pattern(
        "aaa".to_owned(),
        "aa aa aa aa".to_owned()
    ));
}
