struct Solution {}

#[derive(Clone, Copy, PartialEq)]
enum State {
    Unseen,
    LowerOnly,
    UpperAfterLower,
    UpperBeforeLower,
}

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut count_arr = [State::Unseen; 26];

        for &c in word.as_bytes() {
            if c.is_ascii_lowercase() {
                let idx = (c - b'a') as usize;
                match count_arr[idx] {
                    State::Unseen => count_arr[idx] = State::LowerOnly,
                    State::UpperAfterLower => count_arr[idx] = State::UpperBeforeLower,
                    _ => {}
                }
            } else {
                let idx = (c - b'A') as usize;
                match count_arr[idx] {
                    State::LowerOnly => count_arr[idx] = State::UpperAfterLower,
                    State::Unseen => count_arr[idx] = State::UpperBeforeLower,
                    _ => {}
                }
            }
        }

        count_arr
            .iter()
            .filter(|&&x| x == State::UpperAfterLower)
            .count() as i32
    }
}

fn main() {
    assert_eq!(3, Solution::number_of_special_chars("aaAbcBC".to_owned()));
    assert_eq!(1, Solution::number_of_special_chars("AbcbDBdD".to_owned()));
}
