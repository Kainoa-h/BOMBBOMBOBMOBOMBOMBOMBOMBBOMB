impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let len = nums.len();
        let mut outside = nums.iter().sum::<i32>();
        
        if outside == x {
            return 0;
        }

        let mut start = 0;
        let mut min_ops = None::<i32>;
        for (end, &n) in nums.iter().enumerate() {
            outside -= n;
            while start < end && outside < x {
                println!("while remove! out:{}, ", outside);
                outside += nums[start];
                start += 1;
            }
            if outside == x {
                let x = (len - end + start - 1) as i32;
                println!("found! curr min_ops {:?}, x {:?}, start, end {},{}", min_ops, x, start, end);
                min_ops = Some(min_ops.map_or(x, |m| m.min(x)));
            }
        }
        min_ops.unwrap_or(-1)
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::min_operations(vec![1, 1, 4, 2, 3], 5), 2);
    assert_eq!(Solution::min_operations(vec![1, 1], 3), -1);
}
