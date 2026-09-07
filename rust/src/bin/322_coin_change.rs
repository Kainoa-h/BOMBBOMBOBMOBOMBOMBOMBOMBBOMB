use std::collections::HashSet;

impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, mut amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }

        coins.sort_unstable_by_key(|x| -x);
        let mut seen_set = HashSet::<i32>::new();
        let mut stack = vec![amount];
        let mut depth = 0;
        while !stack.is_empty() {
            let mut new_stack = Vec::new();
            for amt in stack {
                if amt == 0 {
                    return depth;
                }

                for &c in &coins {
                    let diff = amt - c;
                    if c <= amt && !seen_set.contains(&diff) {
                        seen_set.insert(diff);
                        new_stack.push(diff);
                    }
                }
            }
            stack = new_stack;
            depth += 1;
        }
        -1
    }
}

struct Solution {}
