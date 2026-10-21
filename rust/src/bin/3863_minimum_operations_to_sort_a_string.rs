impl Solution {
    pub fn min_operations(s: String) -> i32 {
        if s.len() <= 1 {
            return 0;
        }
        let bytes = s.as_bytes();
        if s.len() == 2 {
            return if bytes[0] <= bytes[1] { 0 } else { -1 };
        }

        if bytes.windows(2).all(|w| w[0] <= w[1]) {
            return 0;
        }

        if bytes.iter().skip(1).all(|&b| bytes[0] <= b)
            || bytes
                .iter()
                .rev()
                .skip(1)
                .all(|&b| bytes[bytes.len() - 1] >= b)
        {
            return 1;
        }

        if bytes.iter().skip(1).all(|&b| bytes[0] > b)
            && bytes
                .iter()
                .rev()
                .skip(1)
                .all(|&b| bytes[bytes.len() - 1] < b)
        {
            return 3;
        }

        2
    }
}

struct Solution {}
