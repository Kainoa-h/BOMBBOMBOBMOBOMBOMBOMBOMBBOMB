impl Solution {
    pub fn lex_greater_permutation(s: String, target: String) -> String {
        let mut bucket = [0; 256];
        s.as_bytes().iter().for_each(|&b| bucket[b as usize] += 1);
        let next_greater = |bucket: &[usize; 256], c: u8| {
            bucket
                .iter()
                .enumerate()
                .skip(c as usize + 1)
                .take((b'z' - c + 1) as usize)
                .find(|&(_, &count)| count > 0)
                .map(|(i, _)| i)
        };

        let bytes = target.as_bytes();
        let mut result = Vec::new();
        let mut idx = 0;
        let mut backtrack_needed = true;

        while idx < bytes.len() {
            let t_byte = bytes[idx];
            if bucket[t_byte as usize] > 0 {
                bucket[t_byte as usize] -= 1;
                result.push(t_byte);
                idx += 1;
                continue;
            }

            if let Some(next) = next_greater(&bucket, t_byte) {
                result.push(next as u8);
                bucket[next] -= 1;
                backtrack_needed = false;
            }
            break;
        }

        if idx == bytes.len() || backtrack_needed {
            while let Some(p) = result.pop() {
                bucket[p as usize] += 1;
                if let Some(next) = next_greater(&bucket, p) {
                    result.push(next as u8);
                    bucket[next] -= 1;
                    break;
                }
            }
            if result.is_empty() {
                return String::new();
            }
        }

        for (char, &count) in bucket.iter().enumerate().skip(b'a' as usize).take(26) {
            for _ in 0..count {
                result.push(char as u8)
            }
        }

        str::from_utf8(&result).unwrap().to_owned()
    }
}

struct Solution {}

// abzf
// abzd

// zzzz
// zaaa
