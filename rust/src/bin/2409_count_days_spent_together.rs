impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        const fn compute_prefix() -> [i32;13] {
            const DAYS_IN_MTHS: [i32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            let mut prefix = [0;13];
            let mut i = 1;
            let mut sum = 0;
            while i < 13 {
                prefix[i] = sum;
                sum += DAYS_IN_MTHS[i];
                i+=1;
            }
            prefix
        }
        const DAYS_PREFIX: [i32;13] = compute_prefix();
        let to_days = |x: &str| {
            let b = x.as_bytes();
            let mth = ((b[0] - b'0') * 10 + b[1] - b'0') as usize;
            let day = ((b[3] - b'0') * 10 + b[4] - b'0') as i32;
            DAYS_PREFIX[mth] + day
        };
        let start = to_days(&arrive_alice).max(to_days(&arrive_bob));
        let end = to_days(&leave_alice).min(to_days(&leave_bob));
        (end - start + 1).max(0)
    }
}

struct Solution {}
