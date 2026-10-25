impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        const DAYS_IN_MTHS: [i32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let days_prefix = DAYS_IN_MTHS
            .iter()
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect::<Vec<i32>>();
        let to_days = |x: String| {
            let mth = x[0..2].parse::<usize>().unwrap();
            let day = x[3..5].parse::<i32>().unwrap();
            days_prefix[mth - 1] + day
        };
        let arrive_alice = to_days(arrive_alice);
        let leave_alice = to_days(leave_alice);
        let arrive_bob = to_days(arrive_bob);
        let leave_bob = to_days(leave_bob);
        let start = arrive_alice.max(arrive_bob);
        let end = leave_alice.min(leave_bob);
        if end < start {
            0
        } else {
            end - start + 1
        }
    }
}

struct Solution {}
