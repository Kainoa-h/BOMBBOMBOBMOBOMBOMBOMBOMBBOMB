impl Solution {
    pub fn count_commas(mut n: i64) -> i64 {
        if n < 1_000 {
            return 0;
        }
        let mut place = 1_000;
        let mut count = 0;
        while place <= n {
            count += n - place + 1;
            place *= 1_000;
        }
        count
    }
}

struct Solution {}
