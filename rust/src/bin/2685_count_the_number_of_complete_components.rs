impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut dsu = (0..n).collect::<Vec<usize>>();
        let mut dsu_weight = vec![1; n];
        let mut dsu_edges = vec![0; n];

        fn find(dsu: &mut [usize], idx: usize) -> usize {
            if dsu[idx] == idx {
                return idx;
            }
            dsu[idx] = find(dsu, dsu[idx]);
            dsu[idx]
        }

        for edge in edges {
            let (n1, n2) = (edge[0] as usize, edge[1] as usize);
            let (r1, r2) = (find(&mut dsu, n1), find(&mut dsu, n2));
            if r1 == r2 {
                dsu_edges[r1] += 1;
                continue;
            }
            let (root, child) = if dsu_weight[r1] > dsu_weight[r2] {
                (r1, r2)
            } else {
                (r2, r1)
            };

            dsu[child] = root;
            dsu_weight[root] += dsu_weight[child];
            dsu_edges[root] += dsu_edges[child] + 1;
        }

        (0..n)
            .filter(|&x| dsu[x] == x)
            .filter(|&x| {
                let nodes = dsu_weight[x];
                let edges = dsu_edges[x];
                (nodes * (nodes -1))/2 == edges
            }).count() as i32
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
