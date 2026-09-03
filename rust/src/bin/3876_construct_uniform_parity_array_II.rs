impl Solution {
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        let Some(smallest_odd) = nums1.iter().filter(|&&x|x % 2 == 1).min().cloned() else {
            return true;
        };

        let Some(smallest) = nums1.iter().min().cloned() else {
            return false;
        };

        smallest_odd == smallest
    }
}

struct Solution {}
