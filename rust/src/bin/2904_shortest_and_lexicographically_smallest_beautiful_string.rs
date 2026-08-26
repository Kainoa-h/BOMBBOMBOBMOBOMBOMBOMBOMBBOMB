impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let bytes = s.as_bytes();
        let (mut start, mut end) = (0, 0);
        let mut ones = 0;
        let mut substring = None::<&str>;
        while end <= bytes.len() {
            if ones < k {
                if end == bytes.len() {
                    break;
                }
                ones += (b'1' == bytes[end]) as i32;
                end += 1;
                continue;
            }

            if ones == k {
                let candidate = &s[start..end];
                match substring {
                    None => substring = Some(candidate),
                    Some(cur)
                        if cur.len() > candidate.len()
                            || (cur.len() == candidate.len() && cur > candidate) =>
                    {
                        substring = Some(candidate)
                    }
                    _ => {}
                }
            }

            ones -= (b'1' == bytes[start]) as i32;
            start += 1;
        }

        match substring {
            None => "".to_owned(),
            Some(x) => x.to_owned(),
        }
    }
}

struct Solution {}

fn main() {
    // assert_eq!(
    //     Solution::shortest_beautiful_substring("100011001".to_owned(), 3),
    //     "11001".to_owned()
    // );
    // assert_eq!(
    //     Solution::shortest_beautiful_substring("001".to_owned(), 1),
    //     "1".to_owned()
    // );
    // assert_eq!(
    //     Solution::shortest_beautiful_substring("110101000010110101".to_owned(), 3),
    //     "1011".to_owned()
    // );
    assert_eq!(
        Solution::shortest_beautiful_substring("11011".to_owned(), 4),
        "11011".to_owned()
    );
}
