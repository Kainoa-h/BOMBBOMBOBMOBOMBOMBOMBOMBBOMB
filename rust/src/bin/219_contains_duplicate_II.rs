use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut map: HashMap<i32,usize> = HashMap::new();
        for (i,n) in nums.into_iter().enumerate(){
            if let Some(prev_idx) = map.insert(n,i) && i-prev_idx <= k {
                    return true;
            }
        }
        false
    }
}

struct Solution{}

fn main(){
}
