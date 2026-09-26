use std::collections::HashMap;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let dunno = [b'?'];
        let mut k_map = HashMap::new();
        for kv in &knowledge {
            k_map.insert(kv[0].as_bytes(), kv[1].as_bytes());
        }

        let mut result = Vec::<u8>::new();
        let mut start_idx = None::<usize>;
        for (idx, &c) in s.as_bytes().iter().enumerate() {
            if c == b'(' {
                start_idx = Some(idx + 1);
            } else if c == b')' {
                let &value = k_map.get(&s.as_bytes()[start_idx.unwrap()..idx])
                    .unwrap_or(&dunno.as_ref());
                result.extend_from_slice(value);
                start_idx = None;
            } else if start_idx.is_none() {
                result.push(c);
            }
        }

        unsafe { String::from_utf8_unchecked(result) }
    }
}

struct Solution {}
