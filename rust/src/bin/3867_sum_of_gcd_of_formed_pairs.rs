impl Solution {
    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        let gcd = |mut a, mut b| -> i32 {
            while b != 0 {
                (a, b) = (b, a % b);
            }
            a
        };

        let mut prefix_gcd = nums
            .into_iter()
            .scan(0,|max, x|{
                *max = x.max(*max);
                Some(gcd(x, *max))
            })
            .collect::<Vec<i32>>();

        prefix_gcd.sort_unstable();

        let half = prefix_gcd.len() / 2;
        prefix_gcd
            .iter()
            .zip(prefix_gcd.iter().rev())
            .take(half)
            .map(|(&a, &b)| gcd(a, b) as i64)
            .sum()
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::gcd_sum(vec![2, 6, 4]), 2);
}
