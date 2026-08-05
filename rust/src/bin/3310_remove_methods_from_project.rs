impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut call_list = vec![Vec::new(); n];
        for inv in invocations {
            call_list[inv[0] as usize].push(inv[1] as usize);
        }

        let mut sus_list = vec![false; n];
        let mut stack = vec![k as usize];
        sus_list[k as usize] = true;
        while let Some(sus) = stack.pop() {
            for &next in &call_list[sus] {
                if !sus_list[next] {
                    sus_list[next] = true;
                        stack.push(next);
                }
            }
        }

        let mut result = Vec::new();
        for i in 0..n {
            if sus_list[i] {
                continue;
            }
            if call_list[i].iter().any(|&x| sus_list[x]) {
                return (0..n as i32).collect();
            }
            result.push(i as i32);
        }

        result
    }
}

struct Solution {}
