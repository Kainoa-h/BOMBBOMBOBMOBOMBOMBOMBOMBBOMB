impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut output = vec![0; nums.len()];
        let mut pivot_index = nums.iter().filter(|x| **x < pivot).count();
        let mut less_index = 0;
        let mut more_index = nums.iter().filter(|x| **x == pivot).count() + pivot_index;
        for n in nums {
            if n < pivot {
                output[less_index] = n;
                less_index += 1;
            } else if n > pivot {
                output[more_index] = n;
                more_index += 1;
            } else {
                output[pivot_index] = n;
                pivot_index += 1;
            }
        }

        output
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10),
        vec![9, 5, 3, 10, 10, 12, 14]
    );
}
