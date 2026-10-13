impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map = |d: char| -> &'static str {
            match d {
                '2' => "abc",
                '3' => "def",
                '4' => "ghi",
                '5' => "jkl",
                '6' => "mno",
                '7' => "pqrs",
                '8' => "tuv",
                '9' => "wxyz",
                _ => unreachable!(),
            }
        };
        let mut result = Vec::<String>::with_capacity(3_usize.pow(digits.len() as u32));

        for x in map(digits.as_bytes()[0] as char).chars() {
            result.push(x.to_string());
        }

        for c in digits.chars().skip(1) {
            let letters = map(c);
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
