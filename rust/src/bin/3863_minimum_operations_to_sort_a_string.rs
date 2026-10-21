impl Solution {
    pub fn min_operations(s: String) -> i32 {
        if s.len() <= 1 { return 0; }

        let bytes = s.as_bytes();
        if bytes.is_sorted() { return 0; }
        if s.len() == 2 { return -1; }

        let (min_mid, max_mid) = bytes[1..bytes.len() - 1]
            .iter()
            .fold((u8::MAX, u8::MIN), |(min, max), &b| {
                (min.min(b), max.max(b))
            });
        let (first, last) = (bytes[0], bytes[bytes.len() - 1]);

        if first <= last && (first <= min_mid || last >= max_mid) {
            return 1;
        }

        if first > last && (first > max_mid && last < min_mid) {
            return 3;
        }

        2
    }
}

struct Solution {}
