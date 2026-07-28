impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let mut count = [0; 26];
        for &c in s.as_bytes() {
            count[(c - b'a') as usize] += 1;
        }

        let mut left_half = Vec::<u8>::with_capacity(s.len()/2);
        for (i, count) in count.iter().enumerate() {
            let char = i as u8 + b'a';
            left_half.extend(std::iter::repeat_n(char, count/2));
        }

        let mut result = Vec::with_capacity(s.len());
        result.extend(&left_half);
        if s.len() % 2 != 0 {
            result.push(s.as_bytes()[s.len()/2]);
        }
        result.extend(left_half.iter().rev().copied());

        unsafe {
            String::from_utf8_unchecked(result)
        }
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::smallest_palindrome("z".to_string()),
        "z".to_string()
    );
}
