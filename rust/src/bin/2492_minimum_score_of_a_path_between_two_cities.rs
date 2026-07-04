impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut dsu = (0..n as usize+1).map(|x| (x, i32::MAX)).collect::<Vec<(usize, i32)>>();

        fn find(dsu:&mut Vec<(usize, i32)>, idx: usize)-> usize {
            if dsu[idx].0 == idx {
                return idx;
            }
            dsu[idx].0 = find(dsu, dsu[idx].0);
            dsu[idx].0
        }

        for r in roads {
            let (a, b, d) = (r[0] as usize, r[1] as usize, r[2]);
            let (root_a, root_b) = (find(&mut dsu, a), find(&mut dsu, b));
            let (parent, child) = (root_a.min(root_b), root_a.max(root_b));
            let min_dist = d.min(dsu[parent].1).min(dsu[child].1);
            dsu[child] = (parent, min_dist);
            dsu[parent].1 = min_dist;
        }
        dsu[1].1
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::min_score(4, vec![vec![1, 2, 2], vec![1, 3, 4], vec![3, 4, 7]]),
        2
    );

    assert_eq!(
        Solution::min_score(
            13,
            vec![
                vec![2, 12, 1891],
                vec![10, 9, 4138],
                vec![11, 3, 2007],
                vec![1, 10, 9390],
                vec![12, 8, 1915],
                vec![6, 2, 1098],
                vec![5, 4, 2795],
                vec![3, 13, 4562],
                vec![9, 7, 9202],
                vec![4, 6, 6752],
                vec![8, 11, 1480],
                vec![7, 5, 9827]
            ]
        ),
        1098
    );

    assert_eq!(
        Solution::min_score(
            7,
            vec![
                vec![1, 3, 1484],
                vec![3, 2, 3876],
                vec![2, 4, 6823],
                vec![6, 7, 579],
                vec![5, 6, 4436],
                vec![4, 5, 8830]
            ]
        ),
        579
    );
}
