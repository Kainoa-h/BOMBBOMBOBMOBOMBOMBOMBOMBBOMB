impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut call_list = vec![Vec::new(); n];
        for inv in invocations {
            call_list[inv[0] as usize].push(inv[1] as usize);
        }

        let mut sus_list = vec![false; n];
        let mut stack = vec![k as usize];
        while let Some(sus) = stack.pop() {
            if !sus_list[sus] {
                sus_list[sus] = true;
                stack.append(&mut call_list[sus]);
            }
        }

        for i in 0..n {
            if sus_list[i] {
                continue;
            }
            if call_list[i].iter().any(|&x| sus_list[x]) {
                return (0..n as i32).collect();
            }
        }

        (0..n as i32).filter(|&x| !sus_list[x as usize]).collect()
    }
}

struct Solution {}
