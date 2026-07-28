impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let len = s.len();
        let mid = len / 2;
        let mut result = s.into_bytes();
        result[0..mid].sort_unstable();
        for i in 0..mid {
            result[len - i - 1] = result[i];
        }

        unsafe { String::from_utf8_unchecked(result) }
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::smallest_palindrome("z".to_string()),
        "z".to_string()
    );
}
