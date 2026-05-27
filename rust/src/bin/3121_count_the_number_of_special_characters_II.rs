struct Solution {}
impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        // 0: none
        // 1: lower_only
        // 2: upper_after_lower
        // 3: lower_after_upper
        let mut count = 0;
        let mut count_arr = [0_u8; 26];

        for &c in word.as_bytes() {
            if (c as char).is_lowercase() {
                let idx = (c - b'a') as usize;
                if count_arr[idx] == 0 {
                    count_arr[idx] = 1;
                } else if count_arr[idx] == 2 {
                    count_arr[idx] = 3;
                    count -= 1;
                }
            } else {
                let idx = (c - b'A') as usize;
                if count_arr[idx] == 0 {
                    count_arr[idx] = 3;
                } else if count_arr[idx] == 1 {
                    count_arr[idx] = 2;
                    count += 1;
                }
            }
        }

        count
    }
}

fn main() {
    assert_eq!(3, Solution::number_of_special_chars("aaAbcBC".to_owned()));
    assert_eq!(1, Solution::number_of_special_chars("AbcbDBdD".to_owned()));
}
