impl Solution {
    pub fn lex_greater_permutation(s: String, target: String) -> String {
        let mut bucket = [0; 256];
        s.as_bytes().iter().for_each(|&b| bucket[b as usize] += 1);
        let next_greater = |bucket: &[usize; 256], byte: u8| {
            ((byte + 1)..=b'z').find(|&c| bucket[c as usize] > 0)
        };

        let bytes = target.as_bytes();
        let mut result = Vec::with_capacity(bytes.len());
        let mut backtrack_needed = true;

        for &b in bytes {
            if bucket[b as usize] > 0 {
                bucket[b as usize] -= 1;
                result.push(b);
                continue;
            }

            if let Some(next) = next_greater(&bucket, b) {
                result.push(next);
                bucket[next as usize] -= 1;
                backtrack_needed = false;
            }
            break;
        }

        if backtrack_needed {
            while let Some(p) = result.pop() {
                bucket[p as usize] += 1;
                if let Some(next) = next_greater(&bucket, p) {
                    result.push(next);
                    bucket[next as usize] -= 1;
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

        String::from_utf8(result).unwrap()
    }
}

struct Solution {}

// abzf
// abzd

// zzzz
// zaaa
