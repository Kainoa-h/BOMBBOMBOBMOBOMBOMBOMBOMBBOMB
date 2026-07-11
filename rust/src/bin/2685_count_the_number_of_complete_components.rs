impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj_matrix = vec![0_u64; n];
        for edge in edges {
            let (e1, e2) = (edge[0] as usize, edge[1] as usize);
            let (em1, em2) = (1 << e1, 1 << e2);
            adj_matrix[e1] |= em1 | em2;
            adj_matrix[e2] |= em1 | em2;
        }
        let mut visited = 0_u64;
        let mut count = 0;
        for i in 0..n {
            if (visited & (1 << i)) != 0 {
                continue;
            }

            let mut fully_connected = true;
            let byte = adj_matrix[i];
            visited |= byte;
            for off_set in 0..n {
                let mask = 1 << off_set;
                if (byte & mask) == 0 {
                    continue;
                }
                if adj_matrix[off_set] != byte {
                    fully_connected = false;
                    break;
                }
            }

            count += fully_connected as i32;
        }

        count
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::count_complete_components(
            6,
            vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]]
        ),
        3
    );
    assert_eq!(
        Solution::count_complete_components(
            4,
            vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]
        ),
        0
    );
}
