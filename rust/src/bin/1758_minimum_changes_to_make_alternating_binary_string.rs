struct Solution {}

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut sum = 0_i32;
        let mut flip_state = s.as_bytes()[0] == b'1';
        for c in s.bytes().skip(1) {
            if (flip_state && c == b'1') || (!flip_state && c == b'0') {
                sum += 1;
            }
            flip_state = !flip_state;
        }
        sum.min(s.len() as i32 - sum)
    }
}

fn main() {
    assert_eq!(1, Solution::min_operations("0100".to_string()));
    assert_eq!(0, Solution::min_operations("01".to_string()));
    assert_eq!(2, Solution::min_operations("1111".to_string()));
    assert_eq!(2, Solution::min_operations("11100".to_string()));
    assert_eq!(2, Solution::min_operations("111000".to_string()));
}
