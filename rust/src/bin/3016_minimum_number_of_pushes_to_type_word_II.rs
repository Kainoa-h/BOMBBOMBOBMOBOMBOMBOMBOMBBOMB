impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut bucket = vec![0; 26];
        for c in word.into_bytes() {
            bucket[(c - b'a') as usize] += 1;
        }
        bucket.sort_unstable_by_key(|x| -x);
        bucket.into_iter().enumerate().fold(0, |acc, (i, n)|{
            acc + n * match i {
                i if i < 8 => 1,
                i if i < 16 => 2,
                i if i < 24 => 3,
                _ => 4,
            }
        })
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::minimum_pushes("abcd".to_owned()), 4);
    assert_eq!(Solution::minimum_pushes("xycdefghij".to_owned()), 12);
}
