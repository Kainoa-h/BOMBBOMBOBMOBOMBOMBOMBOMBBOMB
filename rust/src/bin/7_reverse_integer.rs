impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut reversed = 0_i32;
        while x != 0 {
            let digit = x % 10_i32;
            x /= 10;

            match reversed.checked_mul(10).and_then(|m| m.checked_add(digit)) {
                Some(s)=> reversed = s,
                None => return 0
            }
        }
        reversed
    }
}

struct Solution {}
