struct Solution {}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut count = 0;
        for c in s.chars().rev() {
            if c == ' ' && count > 0 {
                break;
            }
            if c != ' ' {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    assert_eq!(Solution::length_of_last_word("Heeello world".to_owned()), 5);
    assert_eq!(
        Solution::length_of_last_word("   fly me   to   the moon  ".to_owned()),
        4
    );
}
