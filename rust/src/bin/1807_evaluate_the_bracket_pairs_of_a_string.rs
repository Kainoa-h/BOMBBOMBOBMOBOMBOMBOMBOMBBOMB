use std::collections::HashMap;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let dunno: &[u8] = b"?";
        let bytes = s.as_bytes();
        let k_map =
            knowledge
                .iter()
                .fold(HashMap::with_capacity(knowledge.len()), |mut map, kv| {
                    map.insert(kv[0].as_bytes(), kv[1].as_bytes());
                    map
                });

        let mut result = Vec::<u8>::with_capacity(s.len());
        let mut start_idx = s.len();
        for (idx, &c) in bytes.iter().enumerate() {
            match c {
                b'(' => start_idx = idx + 1,
                b')' => {
                    let &value = k_map.get(&bytes[start_idx..idx]).unwrap_or(&dunno);
                    result.extend_from_slice(value);
                    start_idx = s.len();
                }
                _ if start_idx == s.len() => result.push(c),
                _ => {}
            }
        }

        unsafe { String::from_utf8_unchecked(result) }
    }
}

struct Solution {}
