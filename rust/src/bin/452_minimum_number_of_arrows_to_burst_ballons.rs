impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|v| v[1]);

        let mut current_end = points[0][1];
        let mut result = 1;
        for v in points.iter().skip(1) {
            if v[0] > current_end {
                result += 1;
                current_end = v[1];
            }
        }
        result
    }
}

struct Solution {}
fn main() {
    assert_eq!(
        Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
        2
    );

    assert_eq!(
        Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]),
        4
    );

    assert_eq!(
        Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]),
        2
    );
    assert_eq!(Solution::find_min_arrow_shots(vec![vec![1, 2147483647]]), 1);

    assert_eq!(
        Solution::find_min_arrow_shots(vec![
            vec![9, 12],
            vec![1, 10],
            vec![4, 11],
            vec![8, 12],
            vec![3, 9],
            vec![6, 9],
            vec![6, 7]
        ]),
        2
    );
}
