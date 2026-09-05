impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let mut postfix = Vec::with_capacity(nums.len());

        let mut smallest = *nums.last().unwrap();
        for &n in nums.iter().rev() {
            smallest = smallest.min(n);
            postfix.push(smallest);
        }
        postfix.reverse();

        let mut largest = *nums.first().unwrap();
        for (idx, n) in nums.into_iter().enumerate() {
            largest = largest.max(n);
            let smallest = postfix[idx];
            let score = largest - smallest;
            if score <= k {
                return idx as i32;
            }
        }
        -1
    }
}

struct Solution {}
