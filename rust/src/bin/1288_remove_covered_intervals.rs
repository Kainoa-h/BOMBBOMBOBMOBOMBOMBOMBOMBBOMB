impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|x| (x[0], -x[1]));
        let mut upperbound = 0;
        let mut kept = 0;
        for interval in intervals {
            let to = interval[1];
            if to > upperbound {
                upperbound = to;
                kept += 1;
            }
        }
        kept
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::remove_covered_intervals(vec![vec![1, 2], vec![1, 4], vec![3, 4]]),
        1
    );
}
