impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut buckets = [0;256];
        let mut start = 0;
        let mut max = 0;
        for (end, &c) in s.iter().enumerate() {
            let c = c as usize;
            buckets[c] += 1;
            while buckets[c] > 2 {
                buckets[s[start] as usize] -= 1;
                start += 1;
            }
            max = max.max(end - start + 1);
        }
        max as i32
    }
}

struct Solution {}
