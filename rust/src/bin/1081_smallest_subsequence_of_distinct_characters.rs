impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let bytes = s.as_bytes();
        let mut last_index_map = [0_usize; 256];
        for (i, &b) in bytes.iter().enumerate() {
            last_index_map[b as usize] = i;
        }

        let mut monotonic_stack = Vec::new();
        let mut currently_contained = [false; 256];
        for (i, &b) in bytes.iter().enumerate() {
            if currently_contained[b as usize] {
                continue;
            }
            while let Some(&top) = monotonic_stack.last() && b < top && last_index_map[top as usize] > i {
                monotonic_stack.pop();
                currently_contained[top as usize] = false;
            }
            monotonic_stack.push(b);
            currently_contained[b as usize] = true;
        }
        unsafe {
            String::from_utf8_unchecked(monotonic_stack)
        }
    }
}

struct Solution {}
