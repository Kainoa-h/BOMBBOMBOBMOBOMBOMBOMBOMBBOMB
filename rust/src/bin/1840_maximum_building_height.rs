impl Solution {
    pub fn max_building(n: i32, mut restrictions: Vec<Vec<i32>>) -> i32 {
        restrictions.push(vec![1, 0]);
        restrictions.push(vec![n, n - 1]);
        restrictions.sort_unstable_by_key(|x| x[0]);

        for idx in 0..restrictions.len() - 1 {
            let (p_idx, p_max) = (restrictions[idx][0], restrictions[idx][1]);
            let (n_idx, n_max) = (restrictions[idx + 1][0], &mut restrictions[idx + 1][1]);
            let dist = n_idx - p_idx;
            *n_max = (p_max + dist).min(*n_max);
        }

        for idx in (1..restrictions.len()).rev() {
            let (p_idx, p_max) = (restrictions[idx][0], restrictions[idx][1]);
            let (n_idx, n_max) = (restrictions[idx - 1][0], &mut restrictions[idx - 1][1]);
            let dist = p_idx - n_idx;
            *n_max = (p_max + dist).min(*n_max);
        }

        let mut max = 0;

        for win in restrictions.windows(2) {
            let (p_idx, p_max) = (win[0][0], win[0][1]);
            let (n_idx, n_max) = (win[1][0], win[1][1]);
            let dist = n_idx - p_idx;
            max = max.max((p_max + n_max + dist) / 2);
        }

        max
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::max_building(
            10,
            vec![
                vec![6, 0],
                vec![5, 2],
                vec![7, 0],
                vec![9, 1],
                vec![2, 4],
                vec![3, 4],
                vec![4, 0],
                vec![8, 2],
                vec![10, 0]
            ]
        ),
        1
    );
}
