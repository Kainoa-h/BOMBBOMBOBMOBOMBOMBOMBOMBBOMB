struct Solution {}

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut check_arr = [false; 58];
        for &b in word.as_bytes() {
            check_arr[(b - b'A') as usize] = true;
        }

        let mut count = 0;
        for i in 0..26 {
            if check_arr[i] && check_arr[i + 32] {
                count += 1;
            }
        }

        count
    }
}

fn main() {}
