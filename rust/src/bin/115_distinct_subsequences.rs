use std::collections::HashMap;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.into_bytes();
        let t = t.into_bytes();
        let mut cache = HashMap::<(usize, usize), i32>::new();
        fn combinations(
            s_idx: usize,
            t_idx: usize,
            s: &[u8],
            t: &[u8],
            cache: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            if t[t_idx..].len() == 1 {
                let x = s[s_idx..].iter().filter(|&&c| c == t[t_idx]).count() as i32;
                return x;
            }

            if s[s_idx..].len() == 1 {
                return 0;
            }

            if let Some(&cached) = cache.get(&(s_idx,t_idx)) {
                return cached
            }
            let mut calls = 0;
            for (idx, &c) in s.iter().enumerate().skip(s_idx) {
                if c == t[t_idx] {
                    let x = combinations(idx + 1, t_idx + 1, s, t, cache);
                    calls += x;
                }
            }
            cache.insert((s_idx,t_idx), calls);
            calls
        }
        combinations(0, 0, &s, &t, &mut cache)
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        5,
        Solution::num_distinct("babgbag".to_owned(), "bag".to_owned())
    );
    assert_eq!(
        3,
        Solution::num_distinct("rabbbit".to_owned(), "rabbit".to_owned())
    );
}
