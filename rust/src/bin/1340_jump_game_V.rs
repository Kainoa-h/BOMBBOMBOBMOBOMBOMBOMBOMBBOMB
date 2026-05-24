struct Solution {}
impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let mut memo = vec![0; arr.len()];

        fn jump(arr: &[i32], memo: &mut [i32], d: usize, index: usize) -> i32 {
            if memo[index] != 0 {
                return memo[index];
            }

            let left_most = index.saturating_sub(d);
            let right_most = (index + d).min(arr.len() - 1);

            let mut max_left = 0;
            for i in (left_most..index).rev() {
                if arr[index] <= arr[i] {
                    break;
                }
                max_left = max_left.max(jump(arr, memo, d, i));
            }
            let mut max_right = 0;
            for i in (index + 1)..=right_most {
                if arr[index] <= arr[i] {
                    break;
                }
                max_right = max_right.max(jump(arr, memo, d, i));
            }

            let visted = max_left.max(max_right) + 1;
            memo[index] = visted;
            visted
        }

        let mut max = 0;
        for i in 0..arr.len() {
            max = max.max(jump(&arr, &mut memo, d as usize, i));
        }

        max
    }
}

fn main() {
    println!("1");
    assert_eq!(
        Solution::max_jumps(vec![6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12], 2),
        4
    );
    println!("2");
    assert_eq!(Solution::max_jumps(vec![3, 3, 3, 3, 3], 3), 1);
    println!("3");
    assert_eq!(Solution::max_jumps(vec![7, 6, 5, 4, 3, 2, 1], 1), 7);
}
