struct Solution {}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut buckets: Vec<i32> = vec![0; citations.len()];
        for &c in &citations {
            if c == 0 {
                continue;
            }
            if c >= citations.len() as i32 {
                buckets[citations.len() - 1] += 1;
            } else {
                println!("{} to index {}", c, c - 1);
                buckets[c as usize - 1] += 1;
            }
        }
        println!("{:?}", buckets);

        let mut sum = 0;
        for (idx, &count) in buckets.iter().enumerate().rev() {
            sum += count;
            println!("current sum: {}, current idx: {}", sum, idx);
            if sum > idx as i32 {
                return idx as i32 + 1; // reset index to 0 based equivilant
            }
        }
        0
    }
}

fn what(citations: Vec<i32>, ans: i32) {
    println!("---{:?}---", citations);
    assert_eq!(Solution::h_index(citations), ans);
}

fn main() {
    what(vec![3, 0, 6, 1, 5], 3);
    what(vec![1, 3, 1], 1);
    what(vec![100], 1);
    what(vec![100, 200], 2);
    what(vec![1, 1, 1], 1);
    what(vec![0, 0, 0], 0);
    what(vec![0, 0, 2], 1);
    what(vec![2, 2, 2], 2);
    what(vec![2, 2, 2, 0], 2);
    what(vec![1, 7, 9, 4], 3);
    what(vec![9, 7, 4, 1], 3);
    what(vec![9, 7, 7, 5, 1], 4);
}
