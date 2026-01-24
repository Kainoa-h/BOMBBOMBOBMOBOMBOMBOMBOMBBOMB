struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let total_len = (m + n) as usize;
        let mut n1 = nums1.clone();
        n1.resize(m as usize, 0_i32);
        let mut n1_iter = n1.iter().peekable();
        let mut n2_iter = nums2.iter().peekable();
        for i in 0..total_len {
            if n1_iter.peek().is_none() {
                nums1[i] = *n2_iter.next().unwrap();
                continue;
            }
            if n2_iter.peek().is_none() {
                nums1[i] = *n1_iter.next().unwrap();
                continue;
            }
            nums1[i] = if n1_iter.peek().unwrap() < n2_iter.peek().unwrap() {
                *n1_iter.next().unwrap()
            } else {
                *n2_iter.next().unwrap()
            };
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
