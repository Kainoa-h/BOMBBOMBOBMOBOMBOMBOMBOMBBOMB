struct Solution {}
impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let cols = mat[0].len() as i32;
        let shift = k % cols;

        for row in mat {
            for i in 0..row.len() {
                let shift_idx = ((i as i32 + shift) % cols) as usize;
                if row[i] != row[shift_idx] {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    assert!(Solution::are_similar(vec![vec![1, 2, 3]], 6));
    assert!(Solution::are_similar(vec![vec![2, 2], vec![2, 2]], 3));
    assert!(!Solution::are_similar(
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9],],
        4
    ));
}
