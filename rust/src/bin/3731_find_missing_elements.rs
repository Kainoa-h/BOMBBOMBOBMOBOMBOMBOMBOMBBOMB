impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut bucket = [false; 101];
        let (mut min, mut max) = (101, 0);
        for n in nums {
            bucket[n as usize] = true;
            max = max.max(n);
            min = min.min(n);
        }
        (min..max).filter(|&x| !bucket[x as usize]).collect()
    }
}

struct Solution {}
