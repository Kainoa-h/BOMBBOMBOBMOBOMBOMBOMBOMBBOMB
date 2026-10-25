impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        const DAYS_IN_MTHS: [usize; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let to_tup = |x: String| (x[0..2].parse::<usize>().unwrap(), x[3..5].parse::<usize>().unwrap());
        let arrive_alice = to_tup(arrive_alice);
        let leave_alice = to_tup(leave_alice);
        let arrive_bob = to_tup(arrive_bob);
        let leave_bob = to_tup(leave_bob);

        let mut result = 0;
        for m in 1..=12 {
            for d in 1..=DAYS_IN_MTHS[m] {
                let date = (m, d);
                if arrive_alice <= date && arrive_bob <= date && leave_alice >= date && leave_bob >= date {
                    result += 1;
                }
            }
        }
        result
    }
}

struct Solution {}
