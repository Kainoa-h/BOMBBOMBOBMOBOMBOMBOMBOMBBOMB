struct Solution {}
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min_len = usize::MAX;
        let mut current_sum = 0;
        let mut start_index = 0_usize;
        for (end_index, &num) in nums.iter().enumerate() {
            current_sum += num;
            while current_sum >= target {
                min_len = min_len.min(end_index - start_index + 1);
                current_sum -= nums[start_index];
                start_index += 1;
            }
        }
        if min_len == usize::MAX {
            0
        } else {
            min_len as i32
        }
    }
}

fn main() {
    println!("Test 1");
    assert_eq!(2, Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]));
    println!("Test 2");
    assert_eq!(1, Solution::min_sub_array_len(3, vec![2, 3, 1, 2, 4, 3]));
    println!("Test 3");
    assert_eq!(1, Solution::min_sub_array_len(3, vec![2, 3]));
    println!("Test 4");
    assert_eq!(1, Solution::min_sub_array_len(1, vec![1]));
    println!("Test 5");
    assert_eq!(3, Solution::min_sub_array_len(11, vec![1, 2, 3, 4, 5]));
}
