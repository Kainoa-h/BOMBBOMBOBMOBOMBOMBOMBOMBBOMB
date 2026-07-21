impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let mut compressed = Vec::new();
        let mut count = 1;
        let s = format!("{};", s);
        for w in s.as_bytes().windows(2) {
            let (prev, now) = (w[0], w[1]);
            if prev == now {
                count += 1;
            } else {
                compressed.push((prev, count));
                count = 1;
            }
        }

        let mut max = 0;
        for w in compressed.windows(3) {
            if w[0].0 != b'0' {
                continue;
            }
            let delta = w[0].1 + w[2].1;
            max = max.max(delta);
        }

        max + compressed
            .into_iter()
            .filter(|x| x.0 == b'1')
            .map(|x| x.1)
            .sum::<i32>()
    }
}

struct Solution {}
