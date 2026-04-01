struct Solution {}
impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        mut healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let n = positions.len();
        let mut indicies: Vec<usize> = (0..n).collect();
        indicies.sort_unstable_by_key(|&a| positions[a]);
        let indicies = indicies;

        let mut stack: Vec<usize> = Vec::new();
        for i in indicies {
            if directions.as_bytes()[i] == b'R' {
                stack.push(i);
                continue;
            }

            while !stack.is_empty() && healths[i] > 0 {
                let left_idx = stack.pop().unwrap();
                if healths[left_idx] > healths[i] {
                    healths[left_idx] -= 1;
                    healths[i] = 0;
                    stack.push(left_idx);
                } else if healths[left_idx] < healths[i] {
                    healths[left_idx] = 0;
                    healths[i] -= 1;
                } else {
                    healths[left_idx] = 0;
                    healths[i] = 0;
                }
            }
        }

        healths.retain(|&x| x > 0);
        healths
    }
}

fn main() {
    println!("-");
    assert_eq!(
        vec![2, 17, 9, 15, 10],
        Solution::survived_robots_healths(
            vec![5, 4, 3, 2, 1],
            vec![2, 17, 9, 15, 10],
            "RRRRR".to_string()
        )
    );

    println!("-");
    assert_eq!(
        vec![14],
        Solution::survived_robots_healths(
            vec![3, 5, 2, 6],
            vec![10, 10, 15, 12],
            "RLRL".to_string()
        )
    );

    println!("-");
    assert!(
        Solution::survived_robots_healths(
            vec![1, 2, 5, 6],
            vec![10, 10, 11, 11],
            "RLRL".to_string()
        )
        .is_empty()
    );

    println!("-");
    assert_eq!(
        vec![47],
        Solution::survived_robots_healths(vec![12, 33, 37], vec![49, 5, 38], "RLL".to_string())
    );
}
