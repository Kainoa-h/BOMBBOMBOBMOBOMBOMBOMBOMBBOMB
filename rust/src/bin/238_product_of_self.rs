struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut output = vec![1; nums.len()];
        for i in 0..nums.len() - 1 {
            output[i + 1] = output[i] * nums[i];
        }
        let mut accumulator = 1;
        for i in (0..nums.len()).rev() {
            output[i] *= accumulator;
            accumulator *= nums[i];
        }

        output
    }
}

fn main() {
    println!("{:?}", Solution::product_except_self(vec![1, 2, 3, 4]));
    println!("Expexted: [24,12,8,6] ");
    println!("{:?}", Solution::product_except_self(vec![-1, 1, 0, -3, 3]));
    println!("Expexted: [0, 0, 9, 0, 0]");
}
