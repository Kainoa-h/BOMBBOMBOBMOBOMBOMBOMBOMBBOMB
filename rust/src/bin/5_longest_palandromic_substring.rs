impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return "".to_owned();
        }
        if s.len() == 1 {
            return s;
        }

        fn find_palindrome(mut left: usize, mut right: usize, s: &str) -> &str {
            while right < s.len() && s.as_bytes()[left] == s.as_bytes()[right] {
                if left == 0 {
                    return &s[0..=right];
                }
                left -= 1;
                right += 1;
            }
            &s[left + 1..right]
        }

        let mut longest = "";
        for i in 0..s.len() - 1 {
            let x = find_palindrome(i, i, &s);
            if longest.len() < x.len() {
                longest = x;
            }
            let y = find_palindrome(i, i + 1, &s);
            if longest.len() < y.len() {
                longest = y;
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
