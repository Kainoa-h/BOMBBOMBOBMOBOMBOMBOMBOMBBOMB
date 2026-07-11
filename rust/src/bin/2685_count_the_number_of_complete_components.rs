impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj_matrix = vec![vec![false; n]; n];
        for edge in edges {
            let (e1, e2) = (edge[0] as usize, edge[1] as usize);
            adj_matrix[e1][e1]= true;
            adj_matrix[e1][e2]= true;
            adj_matrix[e2][e2]= true;
            adj_matrix[e2][e1]= true;
        }
        let mut check_list = vec![false; n];
        let mut count = 0;
        for i in 0..n {
            if check_list[i] {
                continue;
            }

            let mut fully_connected = true;
            for (idx, _conn) in adj_matrix[i].iter().enumerate().filter(|x|*x.1) {
                check_list[idx] = true;
                if adj_matrix[idx] != adj_matrix[i] {
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
