struct Solution {}
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(x) => x as i32,
            None => -1,
        }
    }
}

fn main() {
    assert_eq!(
        Solution::str_str("sadbutsad".to_owned(), "sad".to_owned()),
        0
    );
    assert_eq!(
        Solution::str_str("sadbutsad".to_owned(), "but".to_owned()),
        3
    );
    assert_eq!(
        Solution::str_str("sadbutsad".to_owned(), "ii".to_owned()),
        -1
    );
    assert_eq!(
        Solution::str_str("heheheheh".to_owned(), "ii".to_owned()),
        -1
    );
}
