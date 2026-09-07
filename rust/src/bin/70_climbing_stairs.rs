impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = (1,1);
        for _ in 1..n {
            dp = (dp.1, dp.0 + dp.1);
        }
        dp.1
    }
}

struct Solution {}
