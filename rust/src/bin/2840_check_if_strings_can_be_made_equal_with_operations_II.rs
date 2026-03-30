struct Solution {}
impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {
            return false;
        }

        let mut count = [0; 26];
        for i in (0..s1.len()).step_by(2) {
            count[(s1.as_bytes()[i] - b'a') as usize] += 1;
            count[(s2.as_bytes()[i] - b'a') as usize] -= 1;
        }
        if count.iter().any(|&n| n != 0) {
            return false;
        }

        for i in (1..s1.len()).step_by(2) {
            count[(s1.as_bytes()[i] - b'a') as usize] += 1;
            count[(s2.as_bytes()[i] - b'a') as usize] -= 1;
        }

        count.iter().all(|&n| n == 0)
    }
}

fn main() {
    assert!(Solution::check_strings(
        "abcd".to_string(),
        "cdab".to_string()
    ));
    assert!(Solution::check_strings(
        "abcdba".to_string(),
        "cabdab".to_string()
    ));
    assert!(!Solution::check_strings(
        "abcd".to_string(),
        "dacb".to_string()
    ));
    assert!(!Solution::check_strings(
        "abe".to_string(),
        "bea".to_string()
    ));
}
