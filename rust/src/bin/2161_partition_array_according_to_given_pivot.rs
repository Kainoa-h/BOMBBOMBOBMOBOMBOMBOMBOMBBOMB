impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut less = Vec::new();
        let mut more = Vec::new();
        let mut count = 0;
        for n in nums {
            if n < pivot {
                less.push(n);
            } else if n > pivot {
                more.push(n);
            } else {
                count += 1;
            }
        }

        less.extend(vec![pivot; count]);
        less.extend(more);
        less
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10),
        vec![9, 5, 3, 10, 10, 12, 14]
    );
}
