impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 {
            let mut bucket = [0; 51];
            for &n in &nums {
                bucket[n as usize] += 1;
            }
            nums.into_iter()
                .filter(|&x| bucket[x as usize] == 1)
                .max()
                .unwrap_or(-1)
        } else if k == nums.len() as i32 {
            nums.into_iter().max().unwrap_or(-1)
        } else {
            let first = nums[0];
            let last = *nums.last().unwrap();
            if first == last {
                return -1;
            }
            let (seen_first, seen_last) = nums[1..nums.len()-1].iter().fold((false, false), |(sf, sl), &x|{
                (sf || x == first, sl || x == last)
            });

            let first = if seen_first { -1 } else { first };
            let last = if seen_last { -1 } else { last };
            first.max(last)
        }
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        -1,
        Solution::largest_integer(vec![4, 4, 2, 2, 2, 0, 5, 3, 4, 4], 3)
    );
}
