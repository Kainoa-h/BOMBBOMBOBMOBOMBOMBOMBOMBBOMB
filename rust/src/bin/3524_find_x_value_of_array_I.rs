impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let mut states = vec![0; k as usize];
        let mut counts = vec![0; k as usize];
        for n in nums {
            let mut new_states = vec![0; k as usize];
            new_states[(n % k) as usize] = 1;
            for (r, &state) in states.iter().enumerate() {
                let x = ((r as i64 * n as i64) % k as i64) as usize;
                new_states[x] += state;
            }
            for (count, &state) in counts.iter_mut().zip(&new_states) {
                *count += state;
            }
            states = new_states;
        }
        counts
    }
}

struct Solution {}
