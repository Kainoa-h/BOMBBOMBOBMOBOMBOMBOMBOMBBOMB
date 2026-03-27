struct Solution {}
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        for i in 0..matrix.len() {
            for j in i..matrix.len() {
                (matrix[i][j], matrix[j][i]) = (matrix[j][i], matrix[i][j]);
            }
            matrix[i].reverse();
        }
    }
}

fn main() {
    println!("i ain't writing a test for this");
}
