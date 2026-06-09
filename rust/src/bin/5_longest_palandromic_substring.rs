impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let b = s.as_bytes();
        let find_substring = |mut start: usize, mut end: usize| -> &str {
            while end < b.len() && b[start] == b[end] {
                if start == 0 || end == b.len() - 1 {
                    return &s[start..=end];
                }
                start -= 1;
                end += 1;
            }
            &s[start + 1..end]
        };

        let mut longest = "";
        for idx in 0..s.len() {
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
