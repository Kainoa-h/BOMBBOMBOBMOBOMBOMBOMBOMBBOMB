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

            let mut max_left = (0_usize, 0_i32);
            for i in (left_most..index).rev() {
                if arr[index] <= arr[i] {
                    break;
                }
                if arr[i] > max_left.1 {
                    max_left = (i, arr[i]);
                }
            }
            let mut max_right = (0_usize, 0_i32);
            for i in (index + 1)..=right_most {
                if arr[index] <= arr[i] {
                    break;
                }
                if arr[i] > max_right.1 {
                    max_right = (i, arr[i]);
                }
            }

            if max_left.1 == 0 && max_right.1 == 0 {
                memo[index] = 1;
                return 1;
            }
            let (mut left, mut right) = (0, 0);

            if max_left.1 != 0 {
                left = jump(arr, memo, d, max_left.0);
            }
            if max_right.1 != 0 {
                right = jump(arr, memo, d, max_right.0);
            }
            let visted = left.max(right) + 1;
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
