impl Solution {
    pub fn path_existence_queries(
        n: i32,
        mut nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut prev = nums[0];
        let mut set = 0;
        for x in nums.iter_mut() {
            set += (*x - prev > max_diff) as i32;
            prev = *x;
            *x = set;
        }

        queries
            .iter()
            .map(|x| nums[x[0] as usize] == nums[x[1] as usize])
            .collect()
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::path_existence_queries(2, vec![1, 3], 1, vec![vec![0, 0], vec![0, 1]]),
        vec![true, false]
    );
    assert_eq!(
        Solution::path_existence_queries(
            4,
            vec![2, 5, 6, 8],
            2,
            vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]]
        ),
        vec![false, false, true, true]
    );
}
