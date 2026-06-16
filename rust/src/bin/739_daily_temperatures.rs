impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut monotonic_stack = Vec::with_capacity(temperatures.len());
        let mut result = vec![0; temperatures.len()];
        for i in 0..temperatures.len() {
            while let Some(&top_idx) = monotonic_stack.last()
                && temperatures[top_idx] < temperatures[i]
            {
                monotonic_stack.pop();
                let days_diff = i - top_idx;
                result[top_idx] = days_diff as i32;
            }
            monotonic_stack.push(i);
        }
        result
    }
}
struct Solution {}

fn main() {
    assert_eq!(
        Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
    assert_eq!(
        Solution::daily_temperatures(vec![30, 40, 50, 60]),
        vec![1, 1, 1, 0]
    );
    assert_eq!(
        Solution::daily_temperatures(vec![30, 60, 90]),
        vec![1, 1, 0]
    );
}

/*
Example 1:

Input: temperatures = [73,74,75,71,69,72,76,73]
Output: [1,1,4,2,1,1,0,0]
Example 2:

Input: temperatures = [30,40,50,60]
Output: [1,1,1,0]
Example 3:

Input: temperatures = [30,60,90]
Output: [1,1,0]

*/
