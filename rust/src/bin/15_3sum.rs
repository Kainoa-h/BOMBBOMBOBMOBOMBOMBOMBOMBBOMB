use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut set: HashSet<Vec<i32>> = HashSet::new();
        nums.sort_unstable();
        for f in 1..nums.len() - 1 {
            let mut l = 0;
            let mut r = nums.len() - 1;
            while l < r {
                let sum = nums[l] + nums[f] + nums[r];
                if sum == 0 {
                    let x = vec![nums[l], nums[f], nums[r]];
                    if set.insert(&x) {
                        ans.push(x);
                    }
                    break;
                }
                if sum < 0 {
                    l += 1;
                } else {
                    r -= 1;
                }
                if l == f || r == f {
                    break;
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
