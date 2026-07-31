impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut bucket = [0; 26];
        for c in word.into_bytes() {
            bucket[(c - b'a') as usize] += 1;
        }
        bucket.sort_unstable_by_key(|x| -x);
        bucket
            .chunks(8)
            .enumerate()
            .map(|(i, chunk)| chunk.iter().sum::<i32>() * (i as i32 + 1))
            .sum()
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::minimum_pushes("abcd".to_owned()), 4);
    assert_eq!(Solution::minimum_pushes("xycdefghij".to_owned()), 12);
}
