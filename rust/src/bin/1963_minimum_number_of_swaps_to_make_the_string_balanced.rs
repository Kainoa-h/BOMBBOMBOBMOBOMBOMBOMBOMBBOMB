impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        s.as_bytes()
            .iter()
            .fold((0, 0_u32), |(open, errors), b| match b {
                b'[' => (open + 1, errors),
                _ if open > 0 => (open - 1, errors),
                _ => (open, errors + 1),
            })
            .1
            .div_ceil(2) as i32
    }
}

struct Solution {}
