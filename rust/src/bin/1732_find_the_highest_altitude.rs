impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut alt = 0;
        let mut max_alt = 0;
        for g in gain {
            alt += g;
            max_alt = max_alt.max(alt);
        }

        max_alt
    }
}

struct Solution {}
