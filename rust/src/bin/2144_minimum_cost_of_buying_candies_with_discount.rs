struct Solution {}
impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable_by(|a, b| b.cmp(a));

        let mut min_cost = 0;
        let mut state = 0;
        for c in cost {
            if state == 2 {
                state = 0;
                continue;
            }
            min_cost += c;
            state += 1;
        }
        min_cost
    }
}

fn main() {
    assert_eq!(Solution::minimum_cost(vec![1, 2, 3]), 5);
}
