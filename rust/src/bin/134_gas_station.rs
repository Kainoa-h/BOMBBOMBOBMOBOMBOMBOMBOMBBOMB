struct Solution {}
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total_excess_fuel = 0;
        let mut start_idx = 0;
        let mut fuel_tank = 0;
        for i in 0..gas.len() {
            let spent = gas[i] - cost[i];
            total_excess_fuel += spent;
            fuel_tank += spent;
            if fuel_tank < 0 {
                fuel_tank = 0;
                start_idx = i + 1;
            }
        }

        if total_excess_fuel < 0 {
            -1
        } else {
            start_idx as i32
        }
    }
}

fn what(gas: Vec<i32>, cost: Vec<i32>, ans: i32) {
    println!("\ngas: {:?}", gas);
    println!("cost: {:?}", cost);
    let x = Solution::can_complete_circuit(gas, cost);
    println!("start idx: {:?}", x);
    assert_eq!(x, ans);
}

fn main() {
    what(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2], 3);
    what(vec![2, 3, 4], vec![3, 4, 3], -1);
    what(vec![5, 1, 2, 3, 4], vec![4, 4, 1, 5, 1], 4);
    what(vec![5, 8, 2, 8], vec![6, 5, 6, 6], 3);
}
