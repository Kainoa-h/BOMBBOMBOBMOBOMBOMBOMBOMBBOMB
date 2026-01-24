struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut n1_idx = m - 1;
        let mut n2_idx = n - 1;
        let mut merge_idx = m + n - 1;
        while n2_idx >= 0 {
            if n1_idx >= 0 && nums1[n1_idx as usize] > nums2[n2_idx as usize] {
                nums1[merge_idx as usize] = nums1[n1_idx as usize];
                n1_idx -= 1;
            } else {
                nums1[merge_idx as usize] = nums2[n2_idx as usize];
                n2_idx -= 1;
            }
            merge_idx -= 1;
        }
    }
}
// Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
fn what() {
    let mut m_a = vec![1, 2, 3, 0, 0, 0];
    let mut n_a = vec![2, 5, 6];
    Solution::merge(&mut m_a, 3, &mut n_a, 3);
    print!("{:?}", m_a);
}

fn main() {
    what();
}
