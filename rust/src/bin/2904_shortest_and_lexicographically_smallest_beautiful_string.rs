impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let bytes = s.as_bytes();
        let mut start = 0;
        let mut end = 0;
        let mut ones = 0;
        let mut substring = &s[..];
        let mut substring_binary_value = i64::MAX;
        let mut curr_binary_value = 0;
        while end <= bytes.len() {
            println!();
            if ones < k {
                if end == bytes.len() {
                    break;
                }
                let x = (b'1' == bytes[end]) as i64;
                curr_binary_value <<= 1;
                curr_binary_value += x;
                ones += x as i32;
                end += 1;
                continue;
            }

            if ones == k && (end - start) < substring.len()
                || ((end - start) == substring.len() && curr_binary_value < substring_binary_value)
            {
                substring = &s[start..end];
                substring_binary_value = curr_binary_value;
            }
            println!("substring: {}, value: {}", substring.to_owned(), substring_binary_value);
            println!("start: {}, end: {}, len: {}", start, end, (end-start));

            let x = (b'1' == bytes[start]) as i64;
            println!("x: {}",x);
            curr_binary_value -= x << (end - start - 1);
            ones -= x as i32;
            start += 1;
        }

        if substring_binary_value == i64::MAX {
            "".to_owned()
        } else {
            substring.to_owned()
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
    assert_eq!(
        Solution::shortest_beautiful_substring("110101000010110101".to_owned(), 3),
        "1011".to_owned()
    );
}
