impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let mut count = [0; 26];
        for &c in s.as_bytes() {
            count[(c - b'a') as usize] += 1;
        }

        let mut str_list = vec![0_u8; s.len()];
        let mut head = 0;
        let mut tail = s.len() - 1;
        let mut odd = 0_u8;
        for (i, count) in count.iter().enumerate() {
            let char = i as u8 + b'a';
            if odd == 0 && count % 2 != 0 {
                odd = char;
            }
            let count = count / 2;
            for _ in 0..count {
                str_list[head] = char;
                str_list[tail] = char;
                head += 1;
                tail -= 1;
            }
        }
        if odd != 0 {
            str_list[s.len()/2] = odd;
        }

        str_list.into_iter().map(|x| x as char).collect()
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::smallest_palindrome("z".to_string()),
        "z".to_string()
    );
}
