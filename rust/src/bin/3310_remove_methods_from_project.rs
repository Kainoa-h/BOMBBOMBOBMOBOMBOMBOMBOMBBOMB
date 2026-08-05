impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut call_list = vec![Vec::new(); n];
        let mut flattened_invocations = Vec::with_capacity(invocations.len());
        for inv in &invocations {
            let f = inv[0] as usize;
            let t = inv[1] as usize;
            call_list[f].push(t);
            flattened_invocations.push((f, t));
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
        let cannot_rm = flattened_invocations
            .iter()
            .any(|&x| !sus_list[x.0] && sus_list[x.1]);

        if cannot_rm {
            (0..n as i32).collect()
        } else {
            (0..n as i32).filter(|&x| !sus_list[x as usize]).collect()
        }
    }
}

struct Solution {}
