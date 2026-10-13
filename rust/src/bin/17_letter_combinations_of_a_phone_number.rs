impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        const MAP: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

        fn dfs(digits: &[u8], current: &mut String, result: &mut Vec<String>) {
            if current.len() == digits.len() {
                result.push(current.to_owned());
                return;
            }
            let index = current.len();
            let letters = MAP[(digits[index] - b'2') as usize];
            for l in letters.chars() {
                current.push(l);
                dfs(digits, current, result);
                current.pop();
            }
        }

        let mut result = Vec::<String>::with_capacity(3_usize.pow(digits.len() as u32));
        let mut current = String::with_capacity(digits.len());
        dfs(digits.as_bytes(),&mut current, &mut result);
        result
    }
}

struct Solution {}
