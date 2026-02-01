struct Solution {}
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut l = 0;
        let mut r = s.len() - 1;
        let bytes = s.as_bytes();
        while l < r {
            if !(bytes[l] as char).is_alphanumeric() {
                l += 1;
                continue;
            }
            if !(bytes[r] as char).is_alphanumeric() {
                r -= 1;
                continue;
            }
            let left: String = (bytes[l] as char).to_lowercase().collect();
            let right: String = (bytes[r] as char).to_lowercase().collect();
            if left != right {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}

fn main() {
    assert!(Solution::is_palindrome("tacocat".to_owned()));
    assert!(!Solution::is_palindrome("kitten".to_owned()));
    assert!(Solution::is_palindrome(
        "A man, a plan, a canal: Panama".to_owned()
    ));
}
