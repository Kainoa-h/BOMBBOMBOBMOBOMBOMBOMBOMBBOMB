impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() || s.len() == 1 {
            return s;
        }

        let find_substring = |mut start, mut end| -> &str {
            let b = s.as_bytes();
            if b[start] != b[end] {
                return &s[start..=start];
            }
            while start > 0 && end < s.len() - 1 && b[start - 1] == b[end + 1] {
                start -= 1;
                end += 1;
            }
            &s[start..=end]
        };

        let mut longest = "";
        for idx in 0..s.len() - 1 {
            let even = find_substring(idx, idx + 1);
            if even.len() > longest.len() {
                longest = even;
            }
            let odd = find_substring(idx, idx);
            if odd.len() > longest.len() {
                longest = odd;
            }
        }

        longest.to_owned()
    }
}
struct Solution {}
fn main() {
    println!("{:?}", Solution::longest_palindrome("babad".to_owned()));
    println!("{:?}", Solution::longest_palindrome("cbbd".to_owned()));
    println!("{:?}", Solution::longest_palindrome("a".to_owned()));
    println!("{:?}", Solution::longest_palindrome("aa".to_owned()));
    println!("{:?}", Solution::longest_palindrome("ab".to_owned()));
}
