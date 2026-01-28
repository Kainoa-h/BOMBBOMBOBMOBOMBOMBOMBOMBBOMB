struct Solution {}
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left_idx = match height.iter().position(|x| *x > 0) {
            Some(idx) => idx,
            None => return 0,
        };

        let prefix_vol: Vec<i32> = height
            .iter()
            .scan(0, |acc, &h| {
                *acc += h;
                Some(*acc)
            })
            .collect();

        let mut total_volume = 0;
        let mut left_height = height[left_idx];
        let mut i = left_idx + 1;
        while i < height.len() - 1 {
            let mut vol = 0;
            let mut fallback_height = 0;
            let mut fallback_idx = 0;
            for idx in i.clone()..height.len() {
                let h = height[idx];
                if left_height <= h {
                    i = idx; //will be incremented later
                    vol = ((idx - left_idx - 1) as i32 * left_height)
                        - (prefix_vol[idx - 1] - prefix_vol[left_idx]);
                    left_height = h;
                    left_idx = idx;
                    break;
                }
                if h > fallback_height {
                    fallback_height = h;
                    fallback_idx = idx;
                }
            }
            if vol == 0 && fallback_height != 0 {
                i = fallback_idx;
                vol = ((fallback_idx - left_idx - 1) as i32 * fallback_height)
                    - (prefix_vol[fallback_idx - 1] - prefix_vol[left_idx]);
                left_height = fallback_height;
                left_idx = fallback_idx;
            }
            total_volume += vol;
            i += 1;
        }
        total_volume
    }
}

fn what(height: Vec<i32>, ans: i32) {
    println!("{:?}", height);
    assert_eq!(Solution::trap(height), ans);
}
fn main() {
    // what(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6);
    // what(vec![4, 2, 0, 3, 2, 5], 9);
    // what(vec![4, 2, 3], 1);
    what(vec![5, 4, 1, 2], 1);
}
