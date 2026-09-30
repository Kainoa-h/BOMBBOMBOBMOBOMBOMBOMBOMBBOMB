impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut result = vec![0; seq.len()];
        for x in seq.chars().enumerate().filter(|x| x.1 == '(').step_by(2) {
            result[x.0] = 1;
        }
        for x in seq.chars().enumerate().filter(|x| x.1 == ')').step_by(2) {
            result[x.0] = 1;
        }

        result
    }
}

struct Solution {}
