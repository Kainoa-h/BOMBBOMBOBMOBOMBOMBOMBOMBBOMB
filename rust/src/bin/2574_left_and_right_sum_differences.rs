struct Solution {}
impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let left_sum = nums
            .iter()
            .scan(0, |acc, &n| {
                let x = *acc;
                *acc += n;
                Some(x)
            })
            .collect::<Vec<_>>();

        let mut right_sum = nums
            .iter()
            .rev()
            .scan(0, |acc, &n| {
                let x = *acc;
                *acc += n;
                Some(x)
            })
            .collect::<Vec<_>>();
        right_sum.reverse();

        left_sum
            .iter()
            .zip(right_sum)
            .map(|(l, r)| (l - r).abs())
            .collect::<Vec<_>>()
    }
}

fn main() {
    assert_eq!(
        Solution::left_right_difference(vec![10, 4, 8, 3]),
        vec![15, 1, 11, 22]
    );
    assert_eq!(Solution::left_right_difference(vec![1]), vec![0]);
}
