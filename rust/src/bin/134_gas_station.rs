struct Solution {}
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        // can reuse gas vec but this is easier
        let mut evs: Vec<i32> = Vec::with_capacity(gas.len());
        let mut ev_sum = 0;
        let mut largest_ev = (0, 0);
        for i in 1..gas.len() {
            let ev = gas[i] - cost[i - 1];
            ev_sum += ev;
            evs.push(ev);
            if ev > largest_ev.1 {
                largest_ev = (i - 1, ev);
            }
        }
        let ev = gas[0] - cost.last().unwrap();
        ev_sum += ev;
        evs.push(ev);
        if ev > largest_ev.1 {
            largest_ev = (cost.len() - 1, ev);
        }

        if ev_sum < 0 { -1 } else { largest_ev.0 as i32 }
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
