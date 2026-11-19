impl Solution {
    pub fn minimize_sum(mut nums: Vec<i32>) -> i32 {
        let mut ft = [i32::MAX; 3];
        let mut lt = [i32::MIN; 3];
        for x in nums {
            if x < ft[2] {
                if x < ft[0] {
                    ft = [x, ft[0], ft[1]];
                } else if x < ft[1] {
                    ft = [ft[0], x, ft[1]];
                } else {
                    ft[2] = x;
                }
            }

            if x > lt[0] {
                if x > lt[2] {
                    lt = [lt[1], lt[2], x];
                } else if x > lt[1] {
                    lt = [lt[1], x, lt[2]];
                } else {
                    lt[0] = x;
                }
            }
        }


        let change_first_two = lt[2] - ft[2];
        let change_last_two = lt[0] - ft[0];
        let change_outter_two = lt[1] - ft[1];
        change_first_two.min(change_last_two).min(change_outter_two)
    }
}

struct Solution {}
