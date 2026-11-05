impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut count = arr1.len() as i32;
        for a in arr1 {
            for &b in &arr2 {
                if (a - b).abs() <= d {
                    count -= 1;
                    break;
                }
            }
        }
        count
    }
}

struct Solution {}
