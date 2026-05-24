struct Solution {}
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut count_arr = [0_u32; 26];
        for &c in magazine.as_bytes() {
            count_arr[(c - b'a') as usize] += 1;
        }
        for &c in ransom_note.as_bytes() {
            let i = (c - b'a') as usize;
            if count_arr[i] == 0 {
                return false;
            }
            count_arr[i] -= 1;
        }
        true
    }
}

fn main() {
    assert!(!Solution::can_construct("a".to_owned(), "b".to_owned()));
    assert!(!Solution::can_construct("aa".to_owned(), "ab".to_owned()));
    assert!(Solution::can_construct("aa".to_owned(), "aab".to_owned()));
}
