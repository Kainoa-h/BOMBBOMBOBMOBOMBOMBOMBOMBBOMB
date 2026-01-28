struct Solution {}
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left_idx = 0;
        let mut left_max = 0;
        let mut right_idx = height.len() - 1;
        let mut right_max = 0;
        let mut total_vol = 0;
        while left_idx < right_idx {
            let left_height = height[left_idx];
            let right_height = height[right_idx];
            if left_height < right_height {
                if left_height >= left_max {
                    left_max = left_height;
                } else {
                    total_vol += left_max - left_height;
                }
                left_idx += 1;
            } else {
                if right_height >= right_max {
                    right_max = right_height;
                } else {
                    total_vol += right_max - right_height;
                }
                right_idx -= 1;
            }
        }
        total_vol
    }
}

fn what(height: Vec<i32>, ans: i32) {
    println!("{:?}", height);
    assert_eq!(Solution::trap(height), ans);
}
fn main() {
    what(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6);
    what(vec![4, 2, 0, 3, 2, 5], 9);
    what(vec![4, 2, 3], 1);
    what(vec![5, 4, 1, 2], 1);
}
