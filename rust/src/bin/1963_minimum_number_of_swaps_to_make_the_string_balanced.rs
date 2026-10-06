impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut swaps = 0;
        let (mut left, mut right) = (0, s.len().saturating_sub(1));
        let mut open = 0;
        let mut bytes = s.into_bytes();

        while left < right {
            let c = bytes[left];
            match c {
                b'[' => open += 1,
                _ if open > 0 => open -= 1,
                _ => {
                    while bytes[right] != b'[' {
                        right -= 1;
                    }
                    bytes.swap(left, right);
                    right -= 1;
                    swaps += 1;
                    open += 1;
                }
            }
            left += 1;
        }

        swaps
    }
}

struct Solution {}
