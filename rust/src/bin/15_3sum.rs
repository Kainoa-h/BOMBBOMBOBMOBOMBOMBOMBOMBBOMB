struct Solution {}

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        nums.sort_unstable();
        for f in 0..nums.len() - 2 {
            let mut l = f + 1;
            let mut r = nums.len() - 1;
            if f > 0 && nums[f] == nums[f - 1] {
                continue;
            }
            while l < r {
                let sum = nums[f] + nums[l] + nums[r];
                if sum == 0 {
                    let x = vec![nums[f], nums[l], nums[r]];
                    ans.push(x);
                    l += 1;
                    while l < r && nums[l] == nums[l - 1] {
                        l += 1;
                    }
                    continue;
                }

                if sum < 0 {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
        ans
    }
}

fn what(nums: Vec<i32>, ans: Vec<Vec<i32>>) {
    println!("{:?}", nums);
    assert_eq!(Solution::three_sum(nums), ans);
}

fn main() {
    what(
        vec![-1, 0, 1, 2, -1, -4],
        vec![vec![-1, -1, 2], vec![-1, 0, 1]],
    );
    what(vec![0, 1, 1], vec![]);
    what(vec![0, 0, 0], vec![vec![0, 0, 0]]);
    what(vec![0, 0, 0, 0], vec![vec![0, 0, 0]]);
}
