impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;
        for end in 1..=s.len() {
            for word in &word_dict {
                if end < word.len() {
                    continue;
                }
                let start = end - word.len();
                if dp[start] && &s[start..end] == word {
                    dp[end] = true;
                    break;
                }
            }
        }
        dp[s.len()]
    }
}

struct Solution {}
