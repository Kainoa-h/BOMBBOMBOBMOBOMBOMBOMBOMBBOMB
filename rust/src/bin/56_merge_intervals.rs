impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort();
        let mut result: Vec<Vec<i32>> = Vec::new();

        for inter in intervals {
            if let Some(prev_last) = result.last_mut()
                && prev_last[1] >= inter[0]
            {
                prev_last[1] = prev_last[1].max(inter[1]);
            } else {
                result.push(inter);
            }
        }

        result
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
        vec![vec![1, 6], vec![8, 10], vec![15, 18]]
    )
}
/*
Example 1:

Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
Output: [[1,6],[8,10],[15,18]]
Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
Example 2:

Input: intervals = [[1,4],[4,5]]
Output: [[1,5]]
Explanation: Intervals [1,4] and [4,5] are considered overlapping.
Example 3:

Input: intervals = [[4,7],[1,4]]
Output: [[1,7]]
Explanation: Intervals [1,4] and [4,7] are considered overlapping.
*/
