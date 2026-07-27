use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut set: HashSet<i32> = HashSet::with_capacity(usize::min(nums.len(), k));
        for (i,&n) in nums.iter().enumerate(){
            if i > k {
                set.remove(&nums[i - k - 1]);
            }

            if !set.insert(n){
                    return true;
            }
        }
        false
    }
}

struct Solution{}

fn main(){
}
