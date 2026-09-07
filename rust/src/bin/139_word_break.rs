impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        fn find(idx: usize, s: &String, dict: &Vec<String>, cache: &mut Vec<u8>) -> bool {
            if idx == s.len() {
                return true;
            }

            if cache[idx] == 1 {
                return false;
            }

            for word in dict {
                let end = idx + word.len();
                if s[idx..].len() >= word.len()
                    && s[idx..end] == word[..]
                    && find(end, s, dict, cache)
                {
                    return true;
                }
            }
            cache[idx] = 1;
            false
        }

        let mut cache = vec![0_u8; s.len()];
        find(0, &s, &word_dict, &mut cache)
    }
}

struct Solution {}
