impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns
            .iter()
            .fold(0, |acc, x| acc + word.contains(x) as i32)
    }
}

struct Solution {}
