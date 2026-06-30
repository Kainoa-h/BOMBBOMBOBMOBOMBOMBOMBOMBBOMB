impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut count = 0;
        let mut seen = vec![-1; 3];
        for (i, b) in s.bytes().enumerate() {
            seen[(b - b'a') as usize] = i as i32;
            if seen[0] != -1 && seen[1] != -1 && seen[2] != -1 {
                count += 1 + seen[0].min(seen[1]).min(seen[2]);
            }
        }

        count
    }
}

struct Solution {}

fn main() {}
