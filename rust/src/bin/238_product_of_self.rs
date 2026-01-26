struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix_list: Vec<i32> = Vec::with_capacity(nums.len());
        prefix_list.push(*nums.first().unwrap());
        let mut accumulator = prefix_list[0];
        for &n in nums.iter().skip(1) {
            accumulator *= n;
            prefix_list.push(accumulator);
        }

        let mut postfix_list: Vec<i32> = vec![0; nums.len()];
        *postfix_list.last_mut().unwrap() = *nums.last().unwrap();
        accumulator = *postfix_list.last().unwrap();
        for (idx, &n) in nums.iter().enumerate().rev().skip(1) {
            accumulator *= n;
            postfix_list[idx] = accumulator;
        }

        let mut output = Vec::with_capacity(nums.len());
        output.push(postfix_list[1]);
        for idx in 1..nums.len() - 1 {
            let pre = prefix_list[idx - 1];
            let post = postfix_list[idx + 1];
            output.push(pre * post);
        }
        output.push(prefix_list[prefix_list.len() - 2]);

        output
    }
}

fn main() {
    println!("{:?}", Solution::product_except_self(vec![1, 2, 3, 4]));
    println!("Expexted: [24,12,8,6] ");
    println!("{:?}", Solution::product_except_self(vec![-1, 1, 0, -3, 3]));
    println!("Expexted: [0, 0, 9, 0, 0]");
}
