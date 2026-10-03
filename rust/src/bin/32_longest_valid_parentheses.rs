impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut longest = 0;
        s.as_bytes().iter().map(|&s| if s == b'(' {(1,0)} else {(0,1)}).fold((0,0), |mut acc, x|{
            acc = (acc.0 + x.0, acc.1 + x.1);
            if acc.0 == acc.1 {
                longest = longest.max(acc.0 * 2);
            } else if acc.0 < acc.1 {
                acc = (0,0);
            }
            acc
        });
        s.as_bytes().iter().rev().map(|&s| if s == b'(' {(1,0)} else {(0,1)}).fold((0,0), |mut acc, x|{
            acc = (acc.0 + x.0, acc.1 + x.1);
            if acc.0 == acc.1 {
                longest = longest.max(acc.0 * 2);
            } else if acc.0 > acc.1 {
                acc = (0,0);
            }
            acc
        });
        longest
    }
}

struct Solution {}
