impl Solution {
    pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
        let mut sorted = arr.clone();
        sorted.sort_unstable();
        sorted.dedup();
        let n = sorted.len();

        arr.iter()
            .map(|x| (sorted.binary_search(x).unwrap() + 1) as i32)
            .collect()
    }
}

struct Solution {}
