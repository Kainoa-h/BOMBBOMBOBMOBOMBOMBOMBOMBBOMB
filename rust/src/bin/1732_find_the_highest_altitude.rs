impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        gain.iter()
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .max()
            .unwrap()
            .max(0)
    }
}

struct Solution {}
