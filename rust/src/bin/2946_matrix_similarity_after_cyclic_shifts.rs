struct Solution {}
impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let cols = mat[0].len();
        let shift = k as usize % cols;

        mat.iter().all(|row| {
            row.iter()
                .enumerate()
                .all(|(i, c)| *c == row[(i + shift) % cols])
        })
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
