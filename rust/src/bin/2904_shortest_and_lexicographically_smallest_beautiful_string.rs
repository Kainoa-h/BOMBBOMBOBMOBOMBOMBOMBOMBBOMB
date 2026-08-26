impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let bytes = s.as_bytes();
        let mut best = None::<&str>;
        let mut left = 0;
        let mut ones = 0;

        for right in 0..bytes.len() {
            ones += i32::from(b'1' == bytes[right]);
            if ones == k {
                while bytes[left] == b'0' {
                    left += 1;
                }
                let candidate = &s[left..=right];
                if best.is_none_or(|cur| (candidate.len(), candidate) < (cur.len(), cur)) {
                    best = Some(candidate);
                }
                left += 1;
                ones -= 1;
            }
        }

        best.unwrap_or_default().to_owned()
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::shortest_beautiful_substring("100011001".to_owned(), 3),
        "11001".to_owned()
    );
    assert_eq!(
        Solution::shortest_beautiful_substring("001".to_owned(), 1),
        "1".to_owned()
    );
    assert_eq!(
        Solution::shortest_beautiful_substring("110101000010110101".to_owned(), 3),
        "1011".to_owned()
    );
    assert_eq!(
        Solution::shortest_beautiful_substring("11011".to_owned(), 4),
        "11011".to_owned()
    );
}
