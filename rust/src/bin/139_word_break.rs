impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        fn find(idx: usize, s: &String, dict: &Vec<String>, cache: &mut Vec<u8>) -> bool {
            if idx == s.len() {
                return true;
            }

            if cache[idx] == 1 {
                return false;
            }

            let mut found = false;
            for word in dict {
                let end = idx + word.len();
                if !found && s[idx..].len() >= word.len() && s[idx..end] == word[..] {
                    found |= find(end, s, dict, cache);
                }
            }
            cache[idx] = 1;
            found
        }

        let mut cache = vec![0_u8; s.len()];
        find(0, &s, &word_dict, &mut cache)
    }
}

struct Solution {}
