impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut result = Vec::<String>::with_capacity(3_usize.pow(digits.len() as u32));

        for x in map[(digits.as_bytes()[0] - b'2') as usize].chars() {
            result.push(x.to_string());
        }

        for c in digits.chars().skip(1) {
            let letters = map[(c as u8 - b'2') as usize];
            let base_slice = result[0..].to_vec();
            for _ in 1..letters.len() {
                result.extend_from_slice(&base_slice);
            }
            for (slice, c) in result.chunks_mut(base_slice.len()).zip(letters.chars()) {
                slice.iter_mut().for_each(|x| x.push(c));
            }
        }

        result
    }
}

struct Solution {}
