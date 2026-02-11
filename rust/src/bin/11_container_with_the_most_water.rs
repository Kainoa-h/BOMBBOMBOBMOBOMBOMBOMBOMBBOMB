struct Solution {}
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut largest = 0;
        while l < r {
            let left = height[l];
            let right = height[r];
            let vol = left.min(right) * (r - l) as i32;
            largest = largest.max(vol);
            if left < right {
                l += 1;
            } else {
                r -= 1;
            }
        }

        largest
    }
}

fn what(height: Vec<i32>, ans: i32) {
    println!("{:?}", height);
    assert_eq!(Solution::max_area(height), ans)
}

fn main() {
    what(vec![1, 8, 6, 2, 5, 4, 8, 3, 7], 49);
    what(vec![1, 1], 1);
}
