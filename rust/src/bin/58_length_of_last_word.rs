struct Solution {}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end().chars().rev().take_while(|&c| c != ' ').count() as i32
    }
}

fn main() {
    assert_eq!(Solution::length_of_last_word("Heeello world".to_owned()), 5);
    assert_eq!(
        Solution::length_of_last_word("   fly me   to   the moon  ".to_owned()),
        4
    );
}
