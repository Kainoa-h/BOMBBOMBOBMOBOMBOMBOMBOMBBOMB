impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points
            .iter()
            .map(|v| [v[0] as i64, v[1] as i64])
            .collect::<Vec<[i64; 2]>>();
        points.sort_unstable_by_key(|v| (v[0], v[1]));
        points.push([i64::MAX, 0]);

        let mut overlap_end = points[0][1];
        let mut result = 0;
        for v in points.iter().skip(1) {
            let (start, end) = (v[0], v[1]);
            overlap_end = overlap_end.min(end);
            if start > overlap_end {
                result += 1;
                overlap_end = v[1];
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
